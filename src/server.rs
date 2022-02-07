use std::collections::HashMap;
use std::fmt::Debug;
use std::os::unix::fs::chroot;
use tonic::{transport::Server, Request, Response, Status};

// These module names are auto-generated by tonic so there's nothing I can do to control the uppercase and lowercase
// This is because tonic's parser thinks the service name DDS is one single word in dds.proto
use dds::dds_server::{Dds, DdsServer};
use dds::{
    CreateNewUserReply, CreateNewUserRequest, LoadStringReply, LoadStringRequest,
    StoreStringRequest, SuccessBool,
};

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use once_cell::sync::OnceCell;
use rand::RngCore;
use tonic::transport::ServerTlsConfig;

pub mod dds {
    tonic::include_proto!("dds");
}

static MAP: OnceCell<Mutex<HashMap<String, Vec<u8>>>> = OnceCell::new();

static JWT_SECRET: OnceCell<[u8; 32]> = OnceCell::new();

#[derive(Debug, Default)]
pub struct MyService {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    role: String,
    user_id: String,
    exp: i64,
}

// Once again, this is "Dds" from the auto-generated code from tonic
#[tonic::async_trait]
impl Dds for MyService {
    async fn store_string(
        &self,
        request: Request<StoreStringRequest>,
    ) -> Result<Response<SuccessBool>, Status> {
        println!("Got a request: {:?}", request);
        println!(
            "The role is: {}\nThe user_id is: {}",
            request.metadata().get("role").unwrap().to_str().unwrap(),
            request.metadata().get("user_id").unwrap().to_str().unwrap()
        );
        let body: StoreStringRequest = request.into_inner();
        let key: String = body.key;
        let value: String = body.value;
        let value = value.into_bytes();

        let status = true;

        let m = MAP.get().unwrap();
        m.lock().unwrap().insert(key, value);
        let reply = SuccessBool {
            success: true.into(),
        };

        Ok(Response::new(reply))
    }

    async fn load_string(
        &self,
        request: Request<LoadStringRequest>,
    ) -> Result<Response<LoadStringReply>, Status> {
        println!("Got a request: {:?}", request);
        println!(
            "The role is: {}",
            request.metadata().get("role").unwrap().to_str().unwrap()
        );
        let body: LoadStringRequest = request.into_inner();
        let key: String = body.key;

        let status = true;

        let m = MAP.get().unwrap();
        let m = m.lock().unwrap().clone();
        let res = m.get(&key);
        let value = match res {
            None => return Err(Status::not_found("this key is not found on the server")),
            Some(bytes) => String::from_utf8(bytes.to_vec()).unwrap(),
        };
        let reply = LoadStringReply {
            value: value.into(),
        };

        Ok(Response::new(reply))
    }

    async fn create_new_user(
        &self,
        request: Request<CreateNewUserRequest>,
    ) -> Result<Response<CreateNewUserReply>, Status> {
        println!("Got a request: {:?}", request);
        let role = request.metadata().get("role").unwrap().to_str().unwrap();
        if role != "admin" {
            return Err(Status::permission_denied(
                "You need to be an admin to create a new user.",
            ));
        }
        let body: CreateNewUserRequest = request.into_inner();
        let expire_time: i64 = body.expire_time;
        let mut rng = secp256k1::rand::thread_rng();
        let secp = secp256k1::Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        let secret_key = secret_key.serialize_secret();
        let public_key = public_key.serialize();
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &Claims {
                role: "user".to_string(),
                user_id: base64::encode(&public_key),
                exp: expire_time,
            },
            &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET.get().unwrap()),
        )
        .unwrap();
        let reply = CreateNewUserReply {
            secret_key: secret_key.to_vec(),
            public_key: public_key.to_vec(),
            token,
        };
        Ok(Response::new(reply))
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
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret),
    )
    .unwrap();
    token
}

async fn print_admin_token() {
    let token = get_admin_token();
    println!("{}", token);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    MAP.set(Mutex::new(HashMap::<String, Vec<u8>>::new()))
        .unwrap();
    let mut jwt_secret: [u8; 32] = [0; 32];
    let mut rng = rand::thread_rng();

    rng.fill_bytes(&mut jwt_secret);
    // let jwt_secret = String::from_utf8(jwt_secret.to_vec()).unwrap();
    println!("JWT secret: {:?}", jwt_secret);
    JWT_SECRET.set(jwt_secret);

    tokio::spawn(print_admin_token());

    let addr = "127.0.0.1:8080".parse()?;
    let service = MyService::default();
    let service = DdsServer::with_interceptor(service, check_auth);

    /* TLS */
    // reading cert and key of server from disk
    let cert = include_str!("../cfssl/pd-server.pem");
    let key = include_str!("../cfssl/pd-server-key.pem");
    // creating identity from cert and key
    let server_identity = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());

    // https://developers.cloudflare.com/cloudflare-one/identity/devices/mutual-tls-authentication
    let client_ca_cert = tokio::fs::read("cfssl/ca.pem").await?;
    let client_ca_cert = tonic::transport::Certificate::from_pem(client_ca_cert);

    // creating tls config
    let tls = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert);

    Server::builder()
        // .tls_config(ServerTlsConfig::new().identity(server_identity))?
        .tls_config(tls)?
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);

    let token = match req.metadata().get("authorization") {
        Some(t) => {
            println!("The authorization header is: {}", t.to_str().unwrap());
            t.to_str().unwrap()
        }
        None => {
            println!("Debug: No valid auth token");
            return Err(Status::unauthenticated("No valid auth token"));
        }
    };
    let secret = JWT_SECRET.get().unwrap();
    println!("{:#?}", secret);
    let token = match jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    ) {
        Ok(token_data) => token_data,
        Err(e) => {
            println!("Debug: wrong secret. {}", e);
            return Err(Status::unauthenticated("Wrong secret."));
        }
    };

    let mut req = req;
    req.metadata_mut()
        .insert("role", token.claims.role.parse().unwrap());
    req.metadata_mut()
        .insert("user_id", token.claims.user_id.parse().unwrap());

    Ok(req)
}
