use crate::storage::common::{ends_with_reserved_tokens, get_prefix};
use chrono::Utc;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::{debug, warn};

static MAP: OnceCell<Mutex<HashMap<String, Vec<u8>>>> = OnceCell::new();

pub struct BasicStorage;

impl BasicStorage {
    pub fn new() -> Self {
        match MAP.set(Mutex::new(HashMap::new())) {
            Ok(_) => (),
            Err(_) => warn!("BasicStorage is already initialized"),
        }
        Self {}
    }
}

impl Default for BasicStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::storage::common::Storage for BasicStorage {
    fn create(&self, user_id: &str, key_name: &str, value: &[u8]) -> Result<String, String> {
        ends_with_reserved_tokens(key_name)?;
        let timestamp = Utc::now().timestamp_nanos();
        let mut map = MAP.get().unwrap().lock().unwrap();
        let key_path_created = format!("{}::{}@{}", user_id, key_name, timestamp);
        let user_id_key_name = format!("{}::{}", user_id, key_name);
        debug!("{}", user_id_key_name);
        if map.contains_key(&user_id_key_name) {
            return Err(format!("Key name already exists: {}", user_id_key_name));
        }
        map.insert(key_path_created.clone(), value.to_vec());
        map.insert(user_id_key_name, timestamp.to_string().into_bytes());
        let prefix = get_prefix(key_name);
        let keys_directory = format!("{}::{}__keys", user_id, prefix);
        warn!("keys_directory is {}", keys_directory);
        let contains_key = map.contains_key(&keys_directory);
        if contains_key {
            let v = map.get(&keys_directory).unwrap();
            let mut v: Vec<String> = serde_json::from_slice(v).unwrap();
            v.push(key_path_created.clone());
            map.insert(keys_directory, serde_json::to_vec(&v).unwrap());
        } else {
            map.insert(
                keys_directory,
                serde_json::to_vec(&vec![key_path_created.clone()]).unwrap(),
            );
        }

        Ok(key_path_created)
    }

    fn read_from_key_paths(&self, key_paths: &[String]) -> Result<Vec<Option<Vec<u8>>>, String> {
        let map = MAP.get().unwrap().lock().unwrap();
        let mut result = Vec::new();
        for key_path in key_paths {
            ends_with_reserved_tokens(key_path)?;
            let value = map.get(key_path);
            result.push(value.map(|v| v.to_vec()));
        }
        Ok(result)
    }

    fn read_from_key_names(
        &self,
        user_id: &str,
        key_names: &[String],
    ) -> Result<Vec<Option<Vec<u8>>>, String> {
        let map = MAP.get().unwrap().lock().unwrap();
        let mut result = Vec::new();
        for key_name in key_names {
            ends_with_reserved_tokens(key_name)?;
            let key_path = format!("{}::{}", user_id, key_name);
            let timestamp = map.get(&key_path);
            let timestamp = match timestamp {
                Some(v) => v,
                None => {
                    result.push(None);
                    continue;
                }
            };
            let timestamp = String::from_utf8(timestamp.to_vec()).unwrap();
            let value = map.get(&format!("{}::{}@{}", user_id, key_name, timestamp));
            result.push(value.map(|v| v.to_vec()));
        }
        Ok(result)
    }

    fn list_keys(&self, prefix: &str, include_history: bool) -> Result<Vec<String>, String> {
        let map = MAP.get().unwrap().lock().unwrap();
        let res = map.get(&format!("{}:__keys", prefix));
        Ok(match res {
            None => vec![],
            Some(s) => {
                let keys: Vec<String> = serde_json::from_slice(s).unwrap();
                if keys.is_empty() || include_history {
                    keys
                } else {
                    vec![keys
                        .iter()
                        .max_by_key(|k| {
                            let key = k.as_str();
                            let mut parts = key.rsplitn(2, '@');
                            let timestamp = parts.next().unwrap();
                            timestamp.parse::<i64>().unwrap()
                        })
                        .cloned()
                        .unwrap()]
                }
            }
        })
    }

    fn update(&self, user_id: &str, key_name: &str, value: &[u8]) -> Result<String, String> {
        ends_with_reserved_tokens(key_name)?;
        let timestamp = Utc::now().timestamp_nanos();
        let mut map = MAP.get().unwrap().lock().unwrap();
        let key_path_created = format!("{}::{}@{}", user_id, key_name, timestamp);
        let user_id_key_name = format!("{}::{}", user_id, key_name);
        map.insert(key_path_created.clone(), value.to_vec());
        map.insert(user_id_key_name, timestamp.to_string().into_bytes());
        let prefix = get_prefix(key_name);
        let keys_directory = format!("{}::{}__keys", user_id, prefix);
        let contains_key = map.contains_key(&keys_directory);
        if contains_key {
            let v = map.get(&keys_directory).unwrap();
            let mut v: Vec<String> = serde_json::from_slice(v).unwrap();
            v.push(key_path_created.clone());
            map.insert(keys_directory, serde_json::to_vec(&v).unwrap());
        } else {
            map.insert(
                keys_directory,
                serde_json::to_vec(&vec![key_path_created.clone()]).unwrap(),
            );
        }

        Ok(key_path_created)
    }

    fn delete(&self, user_id: &str, key_name: &str) -> Result<String, String> {
        ends_with_reserved_tokens(key_name)?;
        let timestamp = Utc::now().timestamp();
        let mut map = MAP.get().unwrap().lock().unwrap();
        let user_id_key_name = format!("{}::{}", user_id, key_name);
        map.insert(user_id_key_name, timestamp.to_string().into_bytes());
        Ok(format!("{}::{}@{}", user_id, key_name, timestamp))
    }
}
