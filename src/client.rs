pub mod dds {
    tonic::include_proto!("dds");
}

use crate::dds::dds_client::DdsClient;
use crate::dds::*;
use chrono::Duration;
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

    let mut request = tonic::Request::new(ImportUserRequest {
        public_key: public_key_vec,
        signature_timestamp: timestamp,
        signature: signature.serialize_compact().to_vec(),
        expiration_time: (timestamp + Duration::hours(24).num_seconds()) as i64,
    });

    // Replace this with token generated from server upon startup
    let token = MetadataValue::from_static("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlIjoiYWRtaW4iLCJ1c2VyX2lkIjoiX2FkbWluIiwiZXhwIjoxNjQ2NDY4ODIwfQ.fyHi7Huj6py4HMUg5iPCebvwKHYazs7iXypQ_W-RbHY");
    request.metadata_mut().insert("authorization", token);

    let response = client.import_user(request).await?;

    let response: Jwt = response.into_inner();

    let jwt: String = response.jwt;

    let key_name_and_payload_1 = StorageEntry {
        key_name: "test_key_name".to_string(),
        key_path: Default::default(),
        payload: "test_payload".to_string().into_bytes(),
    };

    let key_name_and_payload_2 = StorageEntry {
        key_name: "test_key_name".to_string(),
        key_path: Default::default(),
        payload: "test_different_payload".to_string().into_bytes(),
    };

    let key_name = StorageEntry {
        key_name: "test_key_name".to_string(),
        key_path: Default::default(),
        payload: Default::default(),
    };

    let keys_to_read = StorageEntries { entries: vec![key_name] };

    let mut request = tonic::Request::new(key_name_and_payload_1.clone());

    let user_token = MetadataValue::from_str(&jwt).unwrap();
    request.metadata_mut().insert("authorization", user_token);

    let response = client.create_entry(request).await?;

    let response: StorageEntry = response.into_inner();

    println!("Test: this response should be ok: {:?}", response);

    let mut request = tonic::Request::new(key_name_and_payload_1.clone());

    let user_token = MetadataValue::from_str(&jwt).unwrap();
    request.metadata_mut().insert("authorization", user_token);

    let response = client.create_entry(request).await;

    assert!(response.is_err(), "Test: this response should fail, created same key name twice");

    let mut request = tonic::Request::new(keys_to_read.clone());

    let user_token = MetadataValue::from_str(&jwt).unwrap();
    request.metadata_mut().insert("authorization", user_token);

    let response = client.read_entries(request).await?;

    let response: StorageEntries = response.into_inner();

    let v: String = String::from_utf8(response.entries[0].payload.clone()).unwrap();
    println!("Test: read response should be ok: {:?}", v);

    let mut request = tonic::Request::new(key_name_and_payload_2.clone());

    let user_token = MetadataValue::from_str(&jwt).unwrap();
    request.metadata_mut().insert("authorization", user_token);

    let response = client.update_entry(request).await?;

    let response: StorageEntry = response.into_inner();

    println!("Test: response after update should return key path: {:?}", response);

    let mut request = tonic::Request::new(keys_to_read);

    let user_token = MetadataValue::from_str(&jwt).unwrap();
    request.metadata_mut().insert("authorization", user_token);

    let response = client.read_entries(request).await?;

    let response: StorageEntries = response.into_inner();

    let v: String = String::from_utf8(response.entries[0].payload.clone()).unwrap();
    println!("Test: read response after update should be ok: {:?}", v);





    Ok(())
}
