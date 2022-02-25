/// DDS Storage.
/// `key_path` is in the following format: `{user_id} :: {key_name} @ {timestamp}`
trait Storage {
    /// Create a new entry in the storage. Create inputs a key and a value.
    ///
    /// # Inputs
    /// * `user_id`: The user's public_key, serialized compactly in base64.
    /// * `key_name`: `key_path` = `{user_id} :: {key_name} @ {timestamp}`
    /// * `value`: the value to store in bytes
    /// # Returns
    /// * `Ok(key_path)` if the entry was created successfully.
    /// Notice that this `key_path` is the entire key_path with the `@timestamp` suffix, where the timestamp is the time when the entry is created.
    /// # How it works
    /// Inside storage,
    /// - add entry that maps from `{user_id} :: {key_name}` to current timestamp.
    /// - add entry that maps from `{user_id} :: {key_name} @ {current timestamp}` to `value`.
    /// - append this key to `{longest prefix of key_path whose subsequent character is a ':'}:__keys`
    /// Returns the complete key_path of the new entry, which is `{user_id} :: {key_name} @ {current timestamp}`.
    /// This check for one thing: the token subsequent to the last ':' should not start with two underscores '__'.
    fn create(&self, user_id: &str, key_name: &str, value: &[u8]) -> Result<String, String>;

    /// Read entries in the storage from the given `key_path`s.
    ///
    /// # Inputs
    /// * `key_paths`: A list of `key_path`s in the format of `{user_id} :: {key_name} @ {timestamp}` to read from.
    ///
    /// # Returns
    /// * `Ok(Vec<Option<Vec<u8>>>))` if the entries were read successfully.
    ///
    /// For each entry:
    /// `Some(value)` if the entry was found, `None` if it was not found.
    ///
    /// # How it works
    /// Just read the entries in the storage.
    /// This check for one thing: the token subsequent to the last ':' should not start with two underscores '__'.\
    fn read_from_key_paths(
        &self,
        user_id: &str,
        key_paths: &[String],
    ) -> Result<Vec<Option<Vec<u8>>>, String>;

    /// Read entries in the storage from the given `key_path`s.
    ///
    /// # Inputs
    /// * `key_names`: a list of `key_name`s to read from.
    ///
    /// # Returns
    /// * `Ok(Vec<Option<Vec<u8>>>))` if the entries were read successfully.
    ///
    /// # How it works
    /// 1. Find the latest timestamp stored at `{user_id} :: {key_name}`.
    /// 2. Return the entry at `{user_id} :: {key_name} @ {latest timestamp}`.
    ///
    /// For each entry:
    /// `Some(value)` if the entry was found, `None` if it was not found.
    /// This check for one thing: the token subsequent to the last ':' should not start with two underscores '__'.
    fn read_from_key_names(
        &self,
        user_id: &str,
        key_names: &[String],
    ) -> Result<Vec<Option<Vec<u8>>>, String>;

    /// Returns all keys that starts with `prefix` if `include_history` is true, otherwise the latest key_path for each `key_path@timestamp` (the one with the largest timestamp value) that starts with `prefix`.
    ///
    /// Specifically, this finds all key_paths that starts with `prefix`, followed by a colon (":"), then have no colons in the rest of the key_path.
    /// If list_all = true, then it returns all key_paths that match the above criteria.
    /// Otherwise, it returns the latest one for all key_paths (the one with the largest timestamp after the @) that matches the above criteria.
    ///
    /// **Does not check for permissions**. This is work for the server and the SDK.
    ///
    /// `prefix` has to be none empty, otherwise return error.
    ///
    /// **Only find keys in the current level**. Example:
    /// ```
    /// /*
    /// Key paths:
    /// - A::x @ 1
    /// - A::x:y @ 1
    /// - A::x:y @ 2
    /// A::x:__keys
    /// - A::x:y:z @ 1
    /// list_key("") -> Err
    /// list_key("A") -> Err
    /// list_key("A:") -> ["A::x@1"]
    /// list_key("A::x") -> ["A::x:y@1", "A::x:y@2"]
    /// list_key("A::x", false) -> ["A::x:y@2"]
    /// list_key("A::x:y") -> ["A::x:y:z@1"]
    /// */
    /// ```
    /// Note that if you want to list all keys possessed by a user, you need to pass in a colon after the user_id.
    ///
    /// # How it works
    /// Inside storage, `prefix:__keys` points to a vector that contains all the keys that starts with prefix with no colons following.
    /// So we just return that if `include_history` is false. If `include_history` is true, we find the one with the largest timestamp for each distinct `key_path_prefix` and return them.
    fn list_keys(&self, prefix: &str, include_history: bool) -> Result<Vec<String>, String>;

    /// Updates the value of entry corresponding to `key_path_prefix` to `value`.
    ///
    /// # Inputs
    /// * `user_id`: The user's public_key, serialized compactly in base64.
    /// * `key_name`: `key_path` = `{user_id} :: {key_name} @ {timestamp}`
    /// * `value`: the value to store in bytes
    ///
    /// # Returns
    /// * `Ok(key_path)` if the entry was updated successfully.
    ///
    /// Notice that this `key_path` is the entire key_path with the `@timestamp` suffix, where the timestamp is the time when the entry is updated.
    /// # How it works
    /// Inside storage,
    /// - replace entry that maps from `{user_id}::{key_name}` to current timestamp.
    /// - add entry that maps from `{user_id}::{key_name}@{current timestamp}` to `value`.
    /// - append this key to `{longest prefix of key_path_prefix whose subsequent character is a ':'}:__keys`
    fn update(&self, user_id: &str, key_name: &str, value: &[u8]) -> Result<(), String>;

    /// Updates the value of entry corresponding to `key_path_prefix` to `value`.
    ///
    /// # Inputs
    /// * `user_id`: The user's public_key, serialized compactly in base64.
    /// * `key_name`: `key_path` = `{user_id} :: {key_name} @ {timestamp}`
    /// # Returns
    /// * `Ok(key_path)` if the entry was deleted successfully.
    /// Notice that this `key_path` is the entire key_path with the `@timestamp` suffix, where the timestamp is the time when the entry is deleted.
    /// # How it works
    /// Inside storage, replace entry that maps from `{user_id}::{key_name}` to current timestamp.
    /// Doesn't touch the `__keys` vector. Therefore, it's still possible to obtain the entry that just got deleted by calling `list_keys` with `include_history = true`
    /// and seeking the entry with the latest timestamp.
    /// Now `{user_id}::{key_name}@{current timestamp}` maps to nothing. So during read, we know that a delete operation has occurred.
    fn delete(&self, user_id: &str, key_name: &str) -> Result<(), String>;
}
