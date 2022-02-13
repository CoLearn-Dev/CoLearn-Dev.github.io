pub mod dds {
    tonic::include_proto!("dds");
}

use chrono::Duration;
use crate::dds::dds_client::DdsClient;
use crate::dds::{
    CreateNewUserReply, CreateNewUserRequest, ImportNewUserRequest, LoadStringReply,
    LoadStringRequest, RefreshTokenRequest, StoreStringRequest, SuccessBool, TokenReply,
};
use openssl::sha::sha256;
use secp256k1::{Message, Secp256k1};
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Client mTLS
    let server_root_ca_cert = tokio::fs::read("cfssl/ca.pem").await?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
    let client_cert = tokio::fs::read("cfssl/client.pem").await?;
    let client_key = tokio::fs::read("cfssl/client-key.pem").await?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let channel = Channel::from_static("https://127.0.0.1:8080")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = DdsClient::new(channel);

    // The following are a test for import new user

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut secp256k1::rand::thread_rng());
    let public_key_vec = public_key.serialize().to_vec();
    let mut msg = public_key_vec.clone();
    let timestamp = chrono::Utc::now().timestamp();
    msg.extend_from_slice(&timestamp.to_le_bytes());
    let signature = secp.sign_ecdsa(&Message::from_slice(&sha256(&msg)).unwrap(), &secret_key);

    let mut request = tonic::Request::new(ImportNewUserRequest {
        public_key: public_key_vec,
        current_timestamp: timestamp,
        signature: signature.serialize_compact().to_vec(),
        expire_time: (timestamp + Duration::hours(24).num_seconds()) as i64,
    });

    // Replace this with token generated from server upon startup
    let token = MetadataValue::from_static("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlIjoiYWRtaW4iLCJ1c2VyX2lkIjoiX2FkbWluIiwiZXhwIjoxNjQ0NjE5NzQ1fQ.yEcVtnPfTSxxav2skvqjdOr44yN6h9FjYr4qvWXc2tA");
    request.metadata_mut().insert("authorization", token);

    let response = client.import_new_user(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
