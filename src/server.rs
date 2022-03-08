use std::fmt::Debug;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

use dds_proto::dds_server::{Dds, DdsServer};
use dds_proto::*;

use chrono::TimeZone;
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::storage::basic::BasicStorage;
use crate::storage::common::Storage;
use once_cell::sync::OnceCell;
use openssl::sha::sha256;
use rand::RngCore;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1};
use tonic::metadata::MetadataMap;
use tonic::transport::ServerTlsConfig;
use tracing::{debug, error};

pub mod dds_proto {
    tonic::include_proto!("dds");
}

static JWT_SECRET: OnceCell<[u8; 32]> = OnceCell::new();

static STORAGE: OnceCell<Box<dyn Storage + Send + Sync>> = OnceCell::new();

#[derive(Debug, Default)]
pub struct MyService {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    role: String,
    user_id: String,
    exp: i64,
}

#[tonic::async_trait]
impl Dds for MyService {
    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<Jwt>, Status> {
        debug!("Got a request: {:?}", request);
        Self::check_user_token(request.metadata())?;
        let secret = JWT_SECRET.get().unwrap();
        let token = request.metadata().get("authorization").unwrap().clone();
        let token = token.to_str().unwrap();
        let body: RefreshTokenRequest = request.into_inner();
        let token = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret),
            &jsonwebtoken::Validation::default(),
        )
        .unwrap();
        let token = token.claims;
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &Claims {
                role: token.role,
                user_id: token.user_id,
                exp: body.expiration_time,
            },
            &jsonwebtoken::EncodingKey::from_secret(secret),
        )
        .unwrap();
        let reply = Jwt { jwt: token };
        Ok(Response::new(reply))
    }

    async fn import_user(
        &self,
        request: Request<ImportUserRequest>,
    ) -> Result<Response<Jwt>, Status> {
        Self::check_admin_token(request.metadata())?;
        let body: ImportUserRequest = request.into_inner();
        let mut public_key_vec: Vec<u8> = body.public_key;
        let public_key: PublicKey = match PublicKey::from_slice(&public_key_vec) {
            Ok(pk) => pk,
            Err(e) => {
                return Err(Status::invalid_argument(format!(
                    "The public key could not be decoded in compressed serialized format: {:?}",
                    e
                )))
            }
        };

        let signature_timestamp: i64 = body.signature_timestamp;
        let signature: Vec<u8> = body.signature;
        let signature = match Signature::from_compact(&signature) {
            Ok(sig) => sig,
            Err(e) => {
                return Err(Status::invalid_argument(format!(
                    "The signature could not be decoded in EDCSA: {}",
                    e
                )))
            }
        };

        if chrono::Utc
            .timestamp(signature_timestamp, 0)
            .signed_duration_since(chrono::Utc::now())
            .num_minutes()
            .abs()
            > 10
        {
            return Err(Status::unauthenticated(
                "the timestamp is more than 10 minutes before the current time",
            ));
        }
        public_key_vec.extend_from_slice(&signature_timestamp.to_le_bytes());
        let message = Message::from_slice(&sha256(&public_key_vec)).unwrap();
        let secp = Secp256k1::new();
        match secp.verify_ecdsa(&message, &signature, &public_key) {
            Ok(_) => {}
            Err(e) => {
                return Err(Status::invalid_argument(format!(
                    "Invalid Signature: {}",
                    e
                )))
            }
        }
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &Claims {
                role: "user".to_string(),
                user_id: base64::encode(&public_key.serialize()),
                exp: body.expiration_time,
            },
            &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET.get().unwrap()),
        )
        .unwrap();
        let reply = Jwt { jwt: token };
        Ok(Response::new(reply))
    }

    async fn create_entry(
        &self,
        request: Request<StorageEntry>,
    ) -> Result<Response<StorageEntry>, Status> {
        Self::check_user_or_admin_token(request.metadata())?;
        let user_id = Self::get_user_id(request.metadata());
        let body: StorageEntry = request.into_inner();
        let key_name: String = body.key_name;
        let payload: Vec<u8> = body.payload;
        let storage = STORAGE.get().unwrap();
        match storage.create(&user_id, &key_name, &payload) {
            Ok(key_path) => Ok(Response::new(StorageEntry {
                key_name: Default::default(),
                key_path,
                payload: Default::default(),
            })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn read_entries(
        &self,
        request: Request<StorageEntries>,
    ) -> Result<Response<StorageEntries>, Status> {
        Self::check_user_or_admin_token(request.metadata())?;
        let user_id = Self::get_user_id(request.metadata());
        let body: StorageEntries = request.into_inner();
        let entries: Vec<StorageEntry> = body.entries;
        let storage = STORAGE.get().unwrap();
        let mut key_names_vec: Vec<String> = Vec::new();
        let mut key_paths_vec: Vec<String> = Vec::new();

        for entry in entries {
            let key_path: String = entry.key_path;
            let key_name: String = entry.key_name;
            debug!("key_path is {}\n, key_name is {}\n", key_path, key_name);
            if key_path.is_empty() && key_name.is_empty() {
                return Err(Status::invalid_argument(
                    "both key_path and key_name are empty",
                ));
            } else if !key_path.is_empty() && !key_name.is_empty() {
                return Err(Status::invalid_argument(
                    "both key_path and key_name are not empty",
                ));
            } else if !key_name.is_empty() {
                key_names_vec.push(key_name);
            } else {
                key_paths_vec.push(key_path);
            }
        }

        let mut entries_vec: Vec<StorageEntry> = Vec::new();

        let payload_returned_from_key_paths = match storage.read_from_key_paths(&key_paths_vec) {
            Ok(entries) => entries,
            Err(e) => return Err(Status::internal(e)),
        };
        let payload_returned_from_key_names =
            match storage.read_from_key_names(&user_id, &key_names_vec) {
                Ok(entries) => entries,
                Err(e) => return Err(Status::internal(e)),
            };
        debug!(
            "payload_returned_from_key_paths is {:?}",
            payload_returned_from_key_paths
        );
        debug!(
            "payload_returned_from_key_names is {:?}",
            payload_returned_from_key_names
        );
        for (key_path, payload) in key_paths_vec.iter().zip(payload_returned_from_key_paths) {
            entries_vec.push(StorageEntry {
                key_name: Default::default(),
                key_path: key_path.to_string(),
                payload: match payload {
                    Some(payload) => {
                        debug!(
                            "payload is {:?}",
                            String::from_utf8(payload.clone()).unwrap()
                        );
                        payload
                    }
                    None => Default::default(),
                },
            });
        }
        for (key_name, payload) in key_names_vec.iter().zip(payload_returned_from_key_names) {
            entries_vec.push(StorageEntry {
                key_name: key_name.to_string(),
                key_path: Default::default(),
                payload: match payload {
                    Some(payload) => {
                        debug!(
                            "payload is {:?}",
                            String::from_utf8(payload.clone()).unwrap()
                        );
                        payload
                    }
                    None => Default::default(),
                },
            });
        }

        Ok(Response::new(StorageEntries {
            entries: entries_vec,
        }))
    }

    async fn update_entry(
        &self,
        request: Request<StorageEntry>,
    ) -> Result<Response<StorageEntry>, Status> {
        Self::check_user_or_admin_token(request.metadata())?;
        let user_id = Self::get_user_id(request.metadata());
        let body: StorageEntry = request.into_inner();
        let key_name: String = body.key_name;
        let payload: Vec<u8> = body.payload;
        let storage = STORAGE.get().unwrap();
        let key_path = storage.update(&user_id, &key_name, &payload);

        let key_path = match key_path {
            Ok(key_path) => key_path,
            Err(e) => return Err(Status::internal(e)),
        };

        Ok(Response::new(StorageEntry {
            key_name: Default::default(),
            key_path,
            payload: Default::default(),
        }))
    }

    async fn delete_entry(
        &self,
        request: Request<StorageEntry>,
    ) -> Result<Response<StorageEntry>, Status> {
        Self::check_user_or_admin_token(request.metadata())?;
        let user_id = Self::get_user_id(request.metadata());
        let body: StorageEntry = request.into_inner();
        let key_name: String = body.key_name;
        let storage = STORAGE.get().unwrap();
        let key_path = storage.delete(&user_id, &key_name);

        let key_path = match key_path {
            Ok(key_path) => key_path,
            Err(e) => return Err(Status::internal(e)),
        };

        Ok(Response::new(StorageEntry {
            key_name: Default::default(),
            key_path,
            payload: Default::default(),
        }))
    }

    async fn read_keys(
        &self,
        request: Request<ReadKeysRequest>,
    ) -> Result<Response<StorageEntries>, Status> {
        Self::check_user_or_admin_token(request.metadata())?;
        let user_id = Self::get_user_id(request.metadata());
        let body: ReadKeysRequest = request.into_inner();
        let storage = STORAGE.get().unwrap();
        let prefix: String = body.prefix;
        let include_history: bool = body.include_history;
        if !prefix.starts_with(&user_id) {
            return Err(Status::invalid_argument(
                "prefix must start with the given user_id",
            ));
        }
        let keys = storage.list_keys(&prefix, include_history);
        match keys {
            Ok(key_paths) => {
                let mut ret: Vec<StorageEntry> = Vec::new();
                for key in key_paths {
                    ret.push(StorageEntry {
                        key_name: Default::default(),
                        key_path: key,
                        payload: Default::default(),
                    });
                }
                Ok(Response::new(StorageEntries { entries: ret }))
            }
            Err(e) => Err(Status::aborted(e)),
        }
    }
}

impl MyService {
    pub fn check_admin_token(request_metadata: &MetadataMap) -> Result<(), Status> {
        let role = request_metadata.get("role").unwrap().to_str().unwrap();
        if role != "admin" {
            Err(Status::permission_denied(
                "This procedure requires an admin token, which you did not provide.",
            ))
        } else {
            Ok(())
        }
    }
    pub fn check_user_or_admin_token(request_metadata: &MetadataMap) -> Result<(), Status> {
        let role = request_metadata.get("role").unwrap().to_str().unwrap();
        if role != "admin" && role != "user" {
            Err(Status::permission_denied(
                "This procedure needs an admin or user token, which you did not provide.",
            ))
        } else {
            Ok(())
        }
    }
    pub fn check_user_token(request_metadata: &MetadataMap) -> Result<(), Status> {
        let role = request_metadata.get("role").unwrap().to_str().unwrap();
        if role != "user" {
            Err(Status::permission_denied(
                "This procedure needs an admin or user token, which you did not provide.",
            ))
        } else {
            Ok(())
        }
    }
    pub fn get_user_id(request_metadata: &MetadataMap) -> String {
        request_metadata
            .get("user_id")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

fn get_admin_token() -> String {
    let secret = JWT_SECRET.get().unwrap();
    let exp = chrono::Utc::now() + chrono::Duration::hours(48);
    let claims = Claims {
        role: "admin".to_string(),
        user_id: "_admin".to_string(),
        exp: exp.timestamp(),
    };
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret),
    )
    .unwrap()
}

async fn print_admin_token() {
    // This should update every 24 hours in production code, but now we're just writing it to a file.
    let token = get_admin_token();
    std::fs::write("admin_token.txt", token.clone()).unwrap();
    debug!("{}", token);
}

pub async fn init_and_run_server(address: String, port: u16) {
    set_jwt();
    tokio::spawn(print_admin_token());

    let socket_address = format!("{}:{}", address, port).parse().unwrap();
    match run_server(socket_address).await {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}

async fn run_server(socket_address: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let service = MyService::default();
    let service = DdsServer::with_interceptor(service, check_auth);
    /* TLS */
    // reading cert and key of server from disk
    let cert = include_str!("../example-ca-keys/server.pem");
    let key = include_str!("../example-ca-keys/server-key.pem");
    // creating identity from cert and key
    let server_identity = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());

    // https://developers.cloudflare.com/cloudflare-one/identity/devices/mutual-tls-authentication
    let client_ca_cert = tokio::fs::read("example-ca-keys/ca.pem").await?;
    let client_ca_cert = tonic::transport::Certificate::from_pem(client_ca_cert);

    // creating tls config
    let tls = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert);

    Server::builder()
        .tls_config(tls)?
        .add_service(service)
        .serve(socket_address)
        .await?;
    Ok(())
}

fn set_jwt() {
    assert!(STORAGE.set(Box::new(BasicStorage::new())).is_ok());
    let mut jwt_secret: [u8; 32] = [0; 32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut jwt_secret);
    debug!("JWT secret: {:?}", jwt_secret);
    JWT_SECRET.set(jwt_secret).unwrap();
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    debug!("Intercepting request: {:?}", req);

    let token = match req.metadata().get("authorization") {
        Some(t) => {
            debug!("The authorization header is: {}", t.to_str().unwrap());
            t.to_str().unwrap()
        }
        None => {
            debug!("Debug: No valid auth token");
            return Err(Status::unauthenticated("No valid auth token"));
        }
    };
    let secret = JWT_SECRET.get().unwrap();
    let token = match jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    ) {
        Ok(token_data) => token_data,
        Err(e) => {
            return Err(Status::unauthenticated(format!(
                "Debug: wrong secret or token has expired. {}",
                e
            )));
        }
    };

    let mut req = req;
    req.metadata_mut()
        .insert("role", token.claims.role.parse().unwrap());
    req.metadata_mut()
        .insert("user_id", token.claims.user_id.parse().unwrap());

    Ok(req)
}
