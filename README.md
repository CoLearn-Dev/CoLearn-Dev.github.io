# Decentralized Data Science

DDS provides a unified interface for the user, storage, communication, and computation. Extending gRPC, DDS simplifies the development of multi-party protocols and allow implementations in different programming languages to work together consistently. With a unified interface that increases potential data contributors, DDS has the potential to enable larger-scale decentralized data collaboration and unlock the true value of data.

## Start DDS server
Use `cargo run -- --address <address> --port <port>` to start the DDS server.

## Generate mTLS certificates
The DDS server and the clients uses mTLS to communicate. To generate the corresponding certificate and keys for mTLS, we use CFSSL. 

Links for some useful tutorials:
- TLS: https://support.pingcap.com/hc/en-us/articles/360050038113-Create-TLS-Certificates-Using-CFSSL
- mTLS: https://developers.cloudflare.com/cloudflare-one/identity/devices/mutual-tls-authentication/
- github: https://github.com/cloudflare/cfssl

## Test the server

### Using client
Use `cargo test` to run integration tests. See `tests/` for more details.
### Using grpcurl

Alternatively, you can use grpcurl to test the server.

```bash
grpcurl -cacert ./example-ca-keys/ca.pem \
 -cert example-ca-keys/client.pem \
 -key example-ca-keys/client-key.pem \
 -import-path ./proto -proto dds.proto \
 -d '{"key_name": "hi", "payload": "eW9v"}' -H "authorization: REPLACE_WITH_JWT" \
 127.0.0.1:8080 dds.DDS/CreateEntry
```


