# Decentralized Data Science

## Generate Admin token

Run `cargo run --bin "dds-server"` and a JWT token in the format "something.something.something" will be printed to the console.
This is the admin token.

## Create user

With the admin token, you can create a user.

```asm
export ADMIN_JWT_TOKEN="something.something.something"
```

```
grpcurl -cacert ./cfssl/ca.pem -cert cfssl/client.pem -key cfssl/client-key.pem -import-path ./proto -proto dds.proto -d '{"expire_time": "1944195035"}' -H 'authorization: ${ADMIN_JWT_TOKEN}' 127.0.0.1:8080 dds.DDS/CreateNewUser
```

You can adjust the expire timestamp. This timestamp is pretty far away from now.

This will give you a user token in one of the response fields.

You can do other stuff with admin token as well. See code for more details.

## Storage CRUD operations

With the user token, you can do Storage CRUD operations, for example.

```bash
export token="something.something.something"

grpcurl -cacert ./cfssl/ca.pem -cert cfssl/client.pem -key cfssl/client-key.pem -import-path ./proto -proto dds.proto -d '{"key": "hi", "value": "eW9v"}' -H "authorization: ${token}" 127.0.0.1:8080 dds.DDS/CreateEntry

grpcurl -cacert ./cfssl/ca.pem -cert cfssl/client.pem -key cfssl/client-key.pem -import-path ./proto -proto dds.proto -d '{"key": "hello", "value": "bmV3"}' -H "authorization: ${token}" 127.0.0.1:8080 dds.DDS/CreateEntry

grpcurl -cacert ./cfssl/ca.pem -cert cfssl/client.pem -key cfssl/client-key.pem -import-path ./proto -proto dds.proto -d '{"keys": "hi", "keys": "hello", "keys": "no"}' -H "authorization: ${token}" 127.0.0.1:8080 dds.DDS/ReadBatch
```


