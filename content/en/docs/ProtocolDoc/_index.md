---
 title: "gRPC Service Specification" 
 linkTitle: "gRPC Service Specification" 
 weight: 8 
 description: > 
 Read the documentation for gRPC data types and service interfaces. 
--- 

<a name="top"></a>

## Table of Contents

- [dds.proto](#dds-proto)
    - [Decision](#dds-Decision)
    - [Empty](#dds-Empty)
    - [ImportUserRequest](#dds-ImportUserRequest)
    - [Jwt](#dds-Jwt)
    - [MQQueueName](#dds-MQQueueName)
    - [MQURI](#dds-MQURI)
    - [Participant](#dds-Participant)
    - [ReadKeysRequest](#dds-ReadKeysRequest)
    - [RefreshTokenRequest](#dds-RefreshTokenRequest)
    - [RegisterProtocolRequest](#dds-RegisterProtocolRequest)
    - [StorageEntries](#dds-StorageEntries)
    - [StorageEntry](#dds-StorageEntry)
    - [SubscribeMQRequest](#dds-SubscribeMQRequest)
    - [Task](#dds-Task)
  
    - [DDS](#dds-DDS)
  
- [Scalar Value Types](#scalar-value-types)



<a name="dds-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dds.proto



<a name="dds-Decision"></a>

### Decision



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_approved | [bool](#bool) |  | Approved / Rejected |
| is_rejected | [bool](#bool) |  |  |
| reason | [string](#string) |  | Reason |
| signature | [bytes](#bytes) |  | Signature |






<a name="dds-Empty"></a>

### Empty







<a name="dds-ImportUserRequest"></a>

### ImportUserRequest
Import a user from an existing public/secret key pair.

A signature of the public key concatenated with the current timestamp at the time of signing is required to verify the user&#39;s possesion of the secret key and to prevent from replay attacks.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| public_key | [bytes](#bytes) |  | The public key of the user, serialized in compact (default) form. |
| signature_timestamp | [int64](#int64) |  | Unix timestamp of time of signature, in unix timestamp format. Te signature should be generated within 10 minutes of the tiem of signature verification |
| signature | [bytes](#bytes) |  | The EDCSA compact signature of the public key concatenated with little endian encoding of current time. This serves as a challenge response for the user to prevent from replay attacks. |
| expiration_time | [int64](#int64) |  | The expiration time for the token to be generated, in unix timestamp format. |






<a name="dds-Jwt"></a>

### Jwt
JSON Web Token (JWT) that is used to authenticate a user. The JWT the user&#39;s role, user_id - which is the base64 encoding of the public key - and the expiration time.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| jwt | [string](#string) |  |  |






<a name="dds-MQQueueName"></a>

### MQQueueName



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| queue_name | [string](#string) |  | The name of the generated queue for this subscription. |






<a name="dds-MQURI"></a>

### MQURI



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uri | [string](#string) |  | The URI of MQ. |






<a name="dds-Participant"></a>

### Participant



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | The user id of this participant. |
| ptype | [string](#string) |  | Type of this participant in the protocol. |






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
| expiration_time | [int64](#int64) |  | The new expiration time for the token, in unix timestamp format. |






<a name="dds-RegisterProtocolRequest"></a>

### RegisterProtocolRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| protocol_name | [string](#string) |  | The name of this protocol. |
| ptypes | [string](#string) | repeated | The list of possible types of participants. |






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






<a name="dds-SubscribeMQRequest"></a>

### SubscribeMQRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| routing_key | [string](#string) |  | routing_key to be subscribed. |






<a name="dds-Task"></a>

### Task



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| task_id | [string](#string) |  | The id of this task. |
| protocol_name | [string](#string) |  | The protocol of this task. |
| protocol_param | [bytes](#bytes) |  | The protocol parameters (after serialization). |
| participants | [Participant](#dds-Participant) | repeated | The list of participants (initiator should be placed first). |
| parent_task | [string](#string) |  | The task id of the parent task. |
| decisions | [Decision](#dds-Decision) | repeated | The list of signatures (align with participants). |
| status | [string](#string) |  | The status of this task. |
| expiration_time | [int64](#int64) |  | The expiration time for waiting for others&#39; decisions, in unix timestamp format. |





 

 

 


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
| CreateTask | [Task](#dds-Task) | [Task](#dds-Task) | An initiator creates a task. Generate a task_id for this task. Represent user(initiator) to sign a decision for this task. Sync this task with other participants. Send task status to MQ. In request, protocol_name, protocol_param, participants are required. parent_task is optional. In response, only task_id will be included. Require user JWT. |
| ConfirmTask | [Task](#dds-Task) | [Empty](#dds-Empty) | A participant confirms a task. Represent user to sign a decision for this task. Sync the decision to the initiator. Send task status to MQ. In request, task_id is required. Require user JWT. |
| FinishTask | [Task](#dds-Task) | [Empty](#dds-Empty) | A participant finishes a task. Send task status to MQ. In request, task_id is required. Require user JWT. |
| RegisterProtocol | [RegisterProtocolRequest](#dds-RegisterProtocolRequest) | [Empty](#dds-Empty) | Register a protocol on DDS. Create queues of different task statuses for this protocol in MQ. Require user JWT. |
| RequestMQInfo | [Empty](#dds-Empty) | [MQURI](#dds-MQURI) | Request the URI of MQ. Return MQ Information for this user. Require user JWT. |
| SubscribeMQ | [SubscribeMQRequest](#dds-SubscribeMQRequest) | [MQQueueName](#dds-MQQueueName) | Subscribe to specified messages from MQ. TODO Require user JWT. |
| InterCoreSyncTask | [Task](#dds-Task) | [Empty](#dds-Empty) | InterCore RPC. Sync a task. If it receives a task with unknown task_id, then create this task in storage and send task status to MQ. Otherwise, update decisions in storage. If all participants&#39; decisions are received and it is the initiator, sync the decisions to other participants. If all participants&#39; decisions are received, send task status to MQ. |

 



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

