# Decentralized Data Science

DDS provides a unified interface for the user, storage, communication, and computation. Extending gRPC, DDS simplifies the development of multi-party protocols and allow implementations in different programming languages to work together consistently. With a unified interface that increases potential data contributors, DDS has the potential to enable larger-scale decentralized data collaboration and unlock the true value of data.

## Start DDS server
Use `cargo run -- --address <address> --port <port>` to start the DDS server.

For example,
```bash
cargo run -- --address "127.0.0.1" --port 8080
```

## Generate mTLS certificates
The DDS server and the clients uses mTLS to communicate. In the repo, we included a set of example certificate and keys (see `example_ca_keys/`). To generate the corresponding certificate and keys for mTLS, you can use CFSSL. 

Links for some useful tutorials on CFSSL:
- [TLS](https://support.pingcap.com/hc/en-us/articles/360050038113-Create-TLS-Certificates-Using-CFSSL)
- [mTLS](https://developers.cloudflare.com/cloudflare-one/identity/devices/mutual-tls-authentication/)
- [GitHub repo](https://github.com/cloudflare/cfssl)

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


