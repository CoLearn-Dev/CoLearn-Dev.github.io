---
 title:"" 
 linkTitle:"Protocol Documentation" 
 weight: 8 
--- 
# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [Protocol Documentation](#protocol-documentation)
  - [Table of Contents](#table-of-contents)
  - [dds.proto](#ddsproto)
    - [ImportUserRequest](#importuserrequest)
    - [Jwt](#jwt)
    - [ReadKeysRequest](#readkeysrequest)
    - [RefreshTokenRequest](#refreshtokenrequest)
    - [StorageEntries](#storageentries)
    - [StorageEntry](#storageentry)
    - [DDS](#dds)
  - [Scalar Value Types](#scalar-value-types)
  
    - [DDS](#dds-DDS)
  
- [Scalar Value Types](#scalar-value-types)



<a name="dds-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dds.proto



<a name="dds-ImportUserRequest"></a>

### ImportUserRequest
Import a user from an existing public/secret key pair.

A signature of the public key concatenated with the current timestamp at the time of signing is required to verify the user&#39;s possesion of the secret key and to prevent from replay attacks.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| public_key | [bytes](#bytes) |  | The public key of the user, serialized in compact (default) form. |
| signature_timestamp | [int64](#int64) |  | Linux timestamp of time of signature, in linux timestamp format. Te signature should be generated within 10 minutes of the tiem of signature verification |
| signature | [bytes](#bytes) |  | The EDCSA compact signature of the public key concatenated with little endian encoding of current time. This serves as a challenge response for the user to prevent from replay attacks. |
| expiration_time | [int64](#int64) |  | The expiration time for the token to be generated, in linux timestamp format. |






<a name="dds-Jwt"></a>

### Jwt
JSON Web Token (JWT) that is used to authenticate a user. The JWT the user&#39;s role, user_id - which is the base64 encoding of the public key - and the expiration time.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| jwt | [string](#string) |  |  |






<a name="dds-ReadKeysRequest"></a>

### ReadKeysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| prefix | [string](#string) |  | The prefix of the key_path of the entries to be retrieved. |
| include_history | [bool](#bool) |  |  |






<a name="dds-RefreshTokenRequest"></a>

### RefreshTokenRequest
Contains the new expiration time to be set for the generated token.

The old token is contained in the header of this request, under the &#39;authorization&#39; field.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| expiration_time | [int64](#int64) |  | THe new expiration time for the token, in linux timestamp format. |






<a name="dds-StorageEntries"></a>

### StorageEntries
A list of entries.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| entries | [StorageEntry](#dds-StorageEntry) | repeated |  |






<a name="dds-StorageEntry"></a>

### StorageEntry
An entry in the DDS storage.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key_name | [string](#string) |  | The key name of the entry. |
| key_path | [string](#string) |  | The path of this entry. May contain information about the user_id, application_id, key name, and the timestamp of the entry. |
| payload | [bytes](#bytes) |  | The payload (value) of the entry. |





 

 

 


<a name="dds-DDS"></a>

### DDS


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| RefreshToken | [RefreshTokenRequest](#dds-RefreshTokenRequest) | [Jwt](#dds-Jwt) | Given a valid JWT and a expiration timestamp, generates a new JWT token with the expiration time set to the input timestamp. Requires user jwt. You cannot refresh an admin token. |
| ImportUser | [ImportUserRequest](#dds-ImportUserRequest) | [Jwt](#dds-Jwt) | Generates a JWT from a user with a public/secret key pair. The generated JWT specify the user&#39;s role as a user, contains their user_id, which is a base64 encoding of the provided public key. Requires admin Jwt. |
| CreateEntry | [StorageEntry](#dds-StorageEntry) | [StorageEntry](#dds-StorageEntry) | Creates an entry in DDS storage. In the entry passed in to the call, the `key_name` field must be nonempty. Every other field is is ignored. Requires user or admin JWT. Returns a key_path with current timestamp included. |
| ReadEntries | [StorageEntries](#dds-StorageEntries) | [StorageEntries](#dds-StorageEntries) | Retrieves entries from DDS storage. One and only one field among `key_name` and `key_path` is nonempty. If both are nonempty, an error is returned. If key_name is nonempty, returns the latest version of the entry with that key name. This is done by first obtaining the timestamp representing the latest version of the entry, and then retrieving the entry with that timestamp by including the timestamp in key_path. In this case, the key_path field is empty in the returned StorageEntry. If key_path is nonempty, returns the entry with the corresponding key path. If you&#39;re looking for a specific version of an entry, use specify the timestamp inside the `key_path` field. In this case, the key_name field is empty in the returned StorageEntry. Requires user or admin JWT. |
| UpdateEntry | [StorageEntry](#dds-StorageEntry) | [StorageEntry](#dds-StorageEntry) | Updates an entry in DDS storage. In the entry passed in to the call, the `key_name` field must be nonempty. Every other field is is ignored. Creates a new entry with the current timestamp in the key_path field. Sets the latest entry to current timestamp. Requires user or admin JWT. Returns a key_path with current timestamp included. |
| DeleteEntry | [StorageEntry](#dds-StorageEntry) | [StorageEntry](#dds-StorageEntry) | Deletes an entry from DDS storage. Sets the latest entry to current timestamp, but unlike UpdateEntry, we do not create a new entry with the current timestamp in the key_path field. Therefore the current timestamp points to nothing. Requires user or admin JWT. Returns a key_path with current timestamp included. |
| ReadKeys | [ReadKeysRequest](#dds-ReadKeysRequest) | [StorageEntries](#dds-StorageEntries) | Returns list of entries in DDS storage whose key_path starts with input prefix. Requires user or admin JWT. |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

