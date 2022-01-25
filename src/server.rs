use std::collections::HashMap;
use std::fmt::Debug;
use tonic::{transport::Server, Request, Response, Status};

// These module names are auto-generated by tonic so there's nothing I can do to control the uppercase and lowercase
// This is because tonic's parser thinks the service name DDS is one single word in dds.proto
use dds::dds_server::{Dds, DdsServer};
use dds::{LoadStringReply, LoadStringRequest, StoreStringRequest, SuccessBool};

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use once_cell::sync::OnceCell;
use tonic::transport::ServerTlsConfig;

pub mod dds {
    tonic::include_proto!("dds");
}

static MAP: OnceCell<Mutex<HashMap<String, Vec<u8>>>> = OnceCell::new();

#[derive(Debug, Default)]
pub struct MyService {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    role: String,
}

// Once again, this is "Dds" from the auto-generated code from tonic
#[tonic::async_trait]
impl Dds for MyService {
    async fn store_string(
        &self,
        request: Request<StoreStringRequest>,
    ) -> Result<Response<SuccessBool>, Status> {
        println!("Got a request: {:?}", request);
        println!("The role is: {}", request.metadata().get("role").unwrap().to_str().unwrap());
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
        println!("The role is: {}", request.metadata().get("role").unwrap().to_str().unwrap());
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    MAP.set(Mutex::new(HashMap::<String, Vec<u8>>::new()))
        .unwrap();

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
    let token = match jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret("password".as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => token_data,
        Err(e) => {

            println!("Debug: wrong secret. {}", e);
            return Err(Status::unauthenticated("Wrong secret (password)"))
        },
    };

    let mut req = req;
    req.metadata_mut().insert("role", token.claims.role.parse().unwrap());

    Ok(req)
}
