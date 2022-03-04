# Decentralized Data Science

## Generating mTLS certificates
We use CFSSL for this. Links for some useful tutorials:
- TLS: https://support.pingcap.com/hc/en-us/articles/360050038113-Create-TLS-Certificates-Using-CFSSL
- mTLS: https://developers.cloudflare.com/cloudflare-one/identity/devices/mutual-tls-authentication/
- github: https://github.com/cloudflare/cfssl 


## Starting the server

Run `cargo run --bin "dds-server"` and a JWT token  will be printed to the console.
This is the admin token.

## Testing the server

### Using client
For testing purposes, the admin token is written to `admin_token.txt` when the server is ran, it is then read by the client when we run `cargo run --bin "dds-client"`.
### Using grpcurl

Alternatively, you can use grpcurl to test the server.

```bash
grpcurl -cacert ./example-ca-keys/ca.pem -cert example-ca-keys/client.pem -key example-ca-keys/client-key.pem -import-path ./proto -proto dds.proto -d '{"key_name": "hi", "payload": "eW9v"}' -H "authorization: REPLACE_WITH_JWT" 127.0.0.1:8080 dds.DDS/CreateEntry
```


