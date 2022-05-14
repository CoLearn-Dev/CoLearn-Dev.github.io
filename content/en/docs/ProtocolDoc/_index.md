---
 title: "gRPC Service Specification" 
 linkTitle: "gRPC Service Specification" 
 weight: 8 
 description: > 
  Read the documentation for gRPC data types and service interfaces. 
--- 

<a name="top"></a>

## Table of Contents

- [colink.proto](#colink-proto)
    - [CoLinkInternalTaskIDList](#colink-CoLinkInternalTaskIDList)
    - [CoLinkInternalTaskIDWithKeyPath](#colink-CoLinkInternalTaskIDWithKeyPath)
    - [ConfirmTaskRequest](#colink-ConfirmTaskRequest)
    - [CoreInfo](#colink-CoreInfo)
    - [Decision](#colink-Decision)
    - [Empty](#colink-Empty)
    - [Jwt](#colink-Jwt)
    - [MQQueueName](#colink-MQQueueName)
    - [Participant](#colink-Participant)
    - [ReadKeysRequest](#colink-ReadKeysRequest)
    - [RefreshTokenRequest](#colink-RefreshTokenRequest)
    - [StorageEntries](#colink-StorageEntries)
    - [StorageEntry](#colink-StorageEntry)
    - [SubscribeRequest](#colink-SubscribeRequest)
    - [SubscriptionMessage](#colink-SubscriptionMessage)
    - [Task](#colink-Task)
    - [UserConsent](#colink-UserConsent)
  
    - [CoLink](#colink-CoLink)
  
- [Scalar Value Types](#scalar-value-types)



<a name="colink-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## colink.proto



<a name="colink-CoLinkInternalTaskIDList"></a>

### CoLinkInternalTaskIDList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| task_ids_with_key_paths | [CoLinkInternalTaskIDWithKeyPath](#colink-CoLinkInternalTaskIDWithKeyPath) | repeated |  |






<a name="colink-CoLinkInternalTaskIDWithKeyPath"></a>

### CoLinkInternalTaskIDWithKeyPath



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key_path | [string](#string) |  |  |
| task_id | [string](#string) |  |  |






<a name="colink-ConfirmTaskRequest"></a>

### ConfirmTaskRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| task_id | [string](#string) |  | The id of this task. |
| decision | [Decision](#colink-Decision) |  | The decision of this task. |






<a name="colink-CoreInfo"></a>

### CoreInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| mq_uri | [string](#string) |  | The URI of MQ. |
| core_public_key | [bytes](#bytes) |  | The public key of the core, serialized in compact (default) form. |






<a name="colink-Decision"></a>

### Decision



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_approved | [bool](#bool) |  | Approved / Rejected |
| is_rejected | [bool](#bool) |  |  |
| reason | [string](#string) |  | Reason |
| signature | [bytes](#bytes) |  | Signature |
| core_public_key | [bytes](#bytes) |  | Core&#39;s public key |
| user_consent | [UserConsent](#colink-UserConsent) |  | User consent |






<a name="colink-Empty"></a>

### Empty







<a name="colink-Jwt"></a>

### Jwt
JSON Web Token (JWT) that is used to authenticate a user. The JWT the user&#39;s role, user_id - which is the base64 encoding of the public key - and the expiration time.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| jwt | [string](#string) |  |  |






<a name="colink-MQQueueName"></a>

### MQQueueName



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| queue_name | [string](#string) |  | The name of the generated queue for this subscription. |






<a name="colink-Participant"></a>

### Participant



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | The user id of this participant. |
| ptype | [string](#string) |  | Type of this participant in the protocol. |






<a name="colink-ReadKeysRequest"></a>

### ReadKeysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| prefix | [string](#string) |  | The prefix of the key_path of the entries to be retrieved. |
| include_history | [bool](#bool) |  |  |






<a name="colink-RefreshTokenRequest"></a>

### RefreshTokenRequest
Contains the new expiration time to be set for the generated token.

The old token is contained in the header of this request, under the &#39;authorization&#39; field.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| expiration_time | [int64](#int64) |  | The new expiration time for the token, in unix timestamp format. |






<a name="colink-StorageEntries"></a>

### StorageEntries
A list of entries.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| entries | [StorageEntry](#colink-StorageEntry) | repeated |  |






<a name="colink-StorageEntry"></a>

### StorageEntry
An entry in the CoLink storage.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key_name | [string](#string) |  | The key name of the entry. |
| key_path | [string](#string) |  | The path of this entry. May contain information about the user_id, application_id, key name, and the timestamp of the entry. Note that, unlike other timestamps used in the protocol, the timestamp in the key_path is the number of *non-leap-nanoseconds* since January 1, 1970 UT, which is different from the unix timestamp. |
| payload | [bytes](#bytes) |  | The payload (value) of the entry. |






<a name="colink-SubscribeRequest"></a>

### SubscribeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key_name | [string](#string) |  | The key_name of the entry to be subscribed. |
| start_timestamp | [int64](#int64) |  | start_timestamp, in the same format as the timestamp in the storage. Not in unix timestamp format. |






<a name="colink-SubscriptionMessage"></a>

### SubscriptionMessage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| change_type | [string](#string) |  | The type of change of the storage entry. valid values: create/update/delete/in-storage. |
| key_path | [string](#string) |  | The key_path of the storage entry. |
| payload | [bytes](#bytes) |  | The payload (value) of the storage entry. |






<a name="colink-Task"></a>

### Task



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| task_id | [string](#string) |  | The id of this task. |
| protocol_name | [string](#string) |  | The protocol of this task. |
| protocol_param | [bytes](#bytes) |  | The protocol parameters (after serialization). |
| participants | [Participant](#colink-Participant) | repeated | The list of participants (initiator should be placed first). |
| parent_task | [string](#string) |  | The task id of the parent task. |
| require_agreement | [bool](#bool) |  | Whether the task requires the signed decisions from all participants to start. If require_agreement=False, the task starts without confirming replies from others. |
| decisions | [Decision](#colink-Decision) | repeated | The list of decisions (align with participants). |
| status | [string](#string) |  | The status of this task. |
| expiration_time | [int64](#int64) |  | The expiration time for waiting for others&#39; decisions, in unix timestamp format. |






<a name="colink-UserConsent"></a>

### UserConsent
Import a user from an existing public/secret key pair.

A signature by the user is required to verify the user&#39;s consent to let the core represent the user.

The signature message is the concatenation of user public key, timestamp at the time of signing, expiration timestamp, and core public key.
The signature will be saved in user&#39;s storage and get passed in inter-core communication, so other cores can ensure the core is representing the user by checking both public keys.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| public_key | [bytes](#bytes) |  | The public key of the user, serialized in compact (default) form. |
| signature_timestamp | [int64](#int64) |  | Unix timestamp of time of signature, in unix timestamp format. The signature should be generated within 10 minutes of the time of signature verification |
| expiration_timestamp | [int64](#int64) |  | The expiration timestamp for the token to be generated, and for the user consent, in unix timestamp format. We assume the JWT will have the same expiration time as the user consent. |
| signature | [bytes](#bytes) |  | The ECDSA compact signature composed of user public key, timestamp at the time of signing, expiration timestamp, and core public key. The signature represents the user&#39;s consent to let the core represent the user by including the trusted core&#39;s public key. |





 

 

 


<a name="colink-CoLink"></a>

### CoLink


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| RefreshToken | [RefreshTokenRequest](#colink-RefreshTokenRequest) | [Jwt](#colink-Jwt) | Given a valid JWT and a expiration timestamp, generates a new JWT token with the expiration time set to the input timestamp. Requires user jwt. You cannot refresh an admin token. |
| ImportUser | [UserConsent](#colink-UserConsent) | [Jwt](#colink-Jwt) | Generates a JWT from a user with a public/secret key pair. The generated JWT specifies the user&#39;s role as a user, contains their user_id, which is a base64 encoding of the provided public key. Requires admin Jwt. |
| CreateEntry | [StorageEntry](#colink-StorageEntry) | [StorageEntry](#colink-StorageEntry) | Creates an entry in CoLink storage. In the entry passed in to the call, the `key_name` field must be nonempty. Every other field is is ignored. Requires user or admin JWT. Returns a key_path with current timestamp included. |
| ReadEntries | [StorageEntries](#colink-StorageEntries) | [StorageEntries](#colink-StorageEntries) | Retrieves entries from CoLink storage. One and only one field among `key_name` and `key_path` is nonempty. If both are nonempty, an error is returned. If key_name is nonempty, returns the latest version of the entry with that key name. This is done by first obtaining the timestamp representing the latest version of the entry, and then retrieving the entry with that timestamp by including the timestamp in key_path. If key_path is nonempty, returns the entry with the corresponding key path. If you&#39;re looking for a specific version of an entry, use specify the timestamp inside the `key_path` field. In both cases, the key_name field is empty in the returned StorageEntry. key_path and payload are nonempty. If an entry is not found. An error is returned. Note that the returned order of the entries is NOT guaranteed to be the same as the order of the input. Requires user or admin JWT. |
| UpdateEntry | [StorageEntry](#colink-StorageEntry) | [StorageEntry](#colink-StorageEntry) | Updates an entry in CoLink storage. In the entry passed in to the call, the `key_name` field must be nonempty. Every other field is is ignored. Creates a new entry with the current timestamp in the key_path field. Sets the latest entry to current timestamp. Requires user or admin JWT. Returns a key_path with current timestamp included. |
| DeleteEntry | [StorageEntry](#colink-StorageEntry) | [StorageEntry](#colink-StorageEntry) | Deletes an entry from CoLink storage. Sets the latest entry to current timestamp, but unlike UpdateEntry, we do not create a new entry with the current timestamp in the key_path field. Therefore the current timestamp points to nothing. Requires user or admin JWT. Returns a key_path with current timestamp included. |
| ReadKeys | [ReadKeysRequest](#colink-ReadKeysRequest) | [StorageEntries](#colink-StorageEntries) | Returns list of entries in CoLink storage whose key_path starts with input prefix. Requires user or admin JWT. |
| CreateTask | [Task](#colink-Task) | [Task](#colink-Task) | An initiator creates a task. Generate a task_id for this task. Represent user(initiator) to sign a decision for this task. Sync this task with other participants. Send task status to MQ. In request, protocol_name, protocol_param, participants are required. parent_task is optional. In response, only task_id will be included. Require user JWT. |
| ConfirmTask | [ConfirmTaskRequest](#colink-ConfirmTaskRequest) | [Empty](#colink-Empty) | A participant confirms a task. Represent user to sign a decision for this task. Sync the decision to the initiator. Send task status to MQ. The task will be ignored if is_approved and is_rejected are both false in the decision. In request, task_id is required. Require user JWT. |
| FinishTask | [Task](#colink-Task) | [Empty](#colink-Empty) | A participant finishes a task. Send task status to MQ. In request, task_id is required. Require user JWT. |
| RequestCoreInfo | [Empty](#colink-Empty) | [CoreInfo](#colink-CoreInfo) | Request the information of the core, including the URI of MQ, and the public key of the core. Return MQ Information optionally and core public key for this user. JWT is optional: when request includes jwt, the uri of mq will be returned. |
| Subscribe | [SubscribeRequest](#colink-SubscribeRequest) | [MQQueueName](#colink-MQQueueName) | Subscribe to changes in the storage. It will let you subscribe to all changes of key_name in storage since start_timestamp. The subscription message is formatted in SubscriptionMessage. Require user JWT. |
| Unsubscribe | [MQQueueName](#colink-MQQueueName) | [Empty](#colink-Empty) | Unsubscribe the changes in the storage. Require user JWT. |
| InterCoreSyncTask | [Task](#colink-Task) | [Empty](#colink-Empty) | InterCore RPC. Sync a task. If it receives a task with unknown task_id, then create this task in storage and send task status to MQ. Otherwise, update decisions in storage. If all participants&#39; decisions are received and it is the initiator, sync the decisions to other participants. If all participants&#39; decisions are received, send task status to MQ. The task status in the request should be ignored even if it exists. |

 



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

