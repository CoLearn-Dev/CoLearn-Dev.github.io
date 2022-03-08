pub mod dds_proto {
    tonic::include_proto!("dds");
}

use ::dds::server::init_and_run;
use chrono::Duration;
use dds_proto::dds_client::DdsClient;
use dds_proto::*;
use openssl::sha::sha256;
use secp256k1::{Message, Secp256k1};
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

async fn generate_request<T>(jwt: &str, data: T) -> tonic::Request<T> {
    let mut request = tonic::Request::new(data);
    let user_token = MetadataValue::from_str(jwt).unwrap();
    request.metadata_mut().insert("authorization", user_token);
    request
}

#[tokio::test]
async fn storage_integration_test() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    tokio::spawn(init_and_run("127.0.0.1".to_string(), 8080));
    test_storage_crud().await;
    Ok(())
}

async fn test_storage_crud() {
    // Client mTLS
    let server_root_ca_cert = tokio::fs::read("example-ca-keys/ca.pem").await.unwrap();
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
    let client_cert = tokio::fs::read("example-ca-keys/client.pem").await.unwrap();
    let client_key = tokio::fs::read("example-ca-keys/client-key.pem")
        .await
        .unwrap();
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    // This is hardcoded because it requires a static string, and there's no way to use format! to generate a static string unless using macros.
    let channel = Channel::from_static("https://127.0.0.1:8080")
        .tls_config(tls)
        .unwrap()
        .connect()
        .await
        .unwrap();

    let mut client = DdsClient::new(channel);

    // Import New user
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

    let token = std::fs::read_to_string("admin_token.txt").unwrap();
    let token = MetadataValue::from_str(&token).unwrap();
    request.metadata_mut().insert("authorization", token);
    let response = client.import_user(request).await.unwrap();
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

    let keys_to_read = StorageEntries {
        entries: vec![key_name.clone()],
    };

    // Create new entry
    let request = generate_request(&jwt, key_name_and_payload_1.clone()).await;
    let response = client.create_entry(request).await.unwrap();
    let response: StorageEntry = response.into_inner();
    println!(
        "Test: The first create entry response should be ok: {:?}",
        response
    );

    // Create same entry, should error
    let responded_key_path: String = response.key_path;
    let request = generate_request(&jwt, key_name_and_payload_1.clone()).await;
    let response = client.create_entry(request).await;
    assert!(
        response.is_err(),
        "Test: this response should fail, created same key name twice"
    );

    // Read entry
    let request = generate_request(&jwt, keys_to_read.clone()).await;
    let response = client.read_entries(request).await.unwrap();
    let response: StorageEntries = response.into_inner();
    let v: String = String::from_utf8(response.entries[0].payload.clone()).unwrap();
    println!("Test: read response should be ok: {:?}", v);

    // Update entry
    let request = generate_request(&jwt, key_name_and_payload_2.clone()).await;
    let response = client.update_entry(request).await.unwrap();
    let response: StorageEntry = response.into_inner();
    println!(
        "Test: response after update should return key path: {:?}",
        response
    );

    // Read entry after update
    let request = generate_request(&jwt, keys_to_read.clone()).await;
    let response = client.read_entries(request).await.unwrap();
    let response: StorageEntries = response.into_inner();
    let v: String = String::from_utf8(response.entries[0].payload.clone()).unwrap();
    println!("Test: read response after update should be ok: {:?}", v);

    let mut keys_to_read2 = keys_to_read.clone();
    keys_to_read2.entries.push(StorageEntry {
        key_name: Default::default(),
        key_path: responded_key_path.clone(),
        payload: Default::default(),
    });

    // Read entries with a key path and a key name
    let request = generate_request(&jwt, keys_to_read2.clone()).await;
    let response = client.read_entries(request).await.unwrap();
    let response: StorageEntries = response.into_inner();
    println!(
        "Test: read response should be now also contain old value: {:?}",
        response
    );

    // Delete entry
    let request = generate_request(&jwt, key_name.clone()).await;
    client.delete_entry(request).await.unwrap();
    let request = generate_request(&jwt, keys_to_read.clone()).await;
    let response = client.read_entries(request).await.unwrap();
    let response: StorageEntries = response.into_inner();
    println!(
        "Test: read response should be empty after deleted: {:?}",
        response
    );

    // Read keys, should contain deleted key(s) both with and without include_history
    let prefix = responded_key_path[0..responded_key_path.find(':').unwrap() + 1].to_string();
    println!("Test: prefix: {:?}", prefix);
    let request = generate_request(
        &jwt,
        ReadKeysRequest {
            include_history: false,
            prefix: prefix.clone(),
        },
    )
    .await;
    let response = client.read_keys(request).await.unwrap();
    let response: StorageEntries = response.into_inner();
    println!(
        "Test: We should see the timestamp of last update: {:?}",
        response
    );

    let request = generate_request(
        &jwt,
        ReadKeysRequest {
            include_history: true,
            prefix: prefix.clone(),
        },
    )
    .await;
    let response = client.read_keys(request).await.unwrap();
    let response: StorageEntries = response.into_inner();
    println!("Test: We should see the all timestamps: {:?}", response);
}
