use chrono::Utc;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

static MAP: OnceCell<Mutex<HashMap<String, Vec<u8>>>> = OnceCell::new();

pub struct DummyStorage;

impl DummyStorage {
    pub fn new() -> Self {
        MAP.set(Mutex::new(HashMap::new()));
        Self {}
    }
}

impl crate::storage::common::Storage for DummyStorage {
    fn create(&self, user_id: &str, key: String, value: &[u8]) -> Result<(), String> {
        if key == "__keys" {
            return Err("__keys is reserved".to_string());
        }
        let timestamp = Utc::now().timestamp();
        let mut map = MAP.get().unwrap().lock().unwrap();
        map.insert(
            format!("{}:__global:__:{}:{}", user_id, key, timestamp),
            value.to_vec(),
        );
        map.insert(
            format!("{}:__global:__:{}", user_id, key),
            timestamp.to_string().into_bytes(),
        );
        let contains_key = map.contains_key(&format!("{}:__global:__:__keys", user_id));
        if contains_key {
            let v = map.get(&format!("{}:__global:__:__keys", user_id)).unwrap();
            let mut v: Vec<String> = serde_json::from_slice(v).unwrap();
            v.push(key);
            map.insert(
                format!("{}:__global:__:__keys", user_id),
                serde_json::to_vec(&v).unwrap(),
            );
        } else {
            map.insert(
                format!("{}:__global:__:__keys", user_id),
                serde_json::to_vec(&vec![key]).unwrap(),
            );
        }

        Ok(())
    }

    fn read(&self, user_id: &str, key: &str) -> Result<Option<Vec<u8>>, String> {
        if key == "__keys" {
            return Err("__keys is reserved".to_string());
        }
        let map = MAP.get().unwrap().lock().unwrap();
        let timestamp = map.get(&format!("{}:__global:__:{}", user_id, key));
        let timestamp = match timestamp {
            Some(v) => v,
            None => return Ok(None),
        };
        let timestamp = String::from_utf8(timestamp.to_vec()).unwrap();
        let value = map.get(&format!("{}:__global:__:{}:{}", user_id, key, timestamp));
        Ok(value.map(|v| v.to_vec()))
    }

    fn read_key_list(&self, user_id: &str) -> Result<Vec<String>, String> {
        let map = MAP.get().unwrap().lock().unwrap();
        let res = map.get(&format!("{}:__global:__:__keys", user_id));
        Ok(match res {
            None => vec![],
            Some(s) => serde_json::from_slice(s).unwrap(),
        })
    }

    fn read_batch(&self, user_id: &str, keys: Vec<String>) -> Result<Vec<Option<Vec<u8>>>, String> {
        let mut res = vec![];
        for key in keys {
            res.push(self.read(user_id, &key).unwrap());
        }
        Ok(res)
    }

    fn update(&self, user_id: &str, key: &str, value: &[u8]) -> Result<(), String> {
        if key == "__keys" {
            return Err("__keys is reserved".to_string());
        }
        let timestamp = Utc::now().timestamp();
        let map = MAP.get().unwrap();
        map.lock().unwrap().insert(
            format!("{}:__global:__:{}:{}", user_id, key, timestamp),
            value.to_vec(),
        );
        map.lock().unwrap().insert(
            format!("{}:__global:__:{}", user_id, key),
            timestamp.to_string().into_bytes(),
        );
        Ok(())
    }

    fn delete(&self, user_id: &str, key: &str) -> Result<(), String> {
        if key == "__keys" {
            return Err("__keys is reserved".to_string());
        }
        let map = MAP.get().unwrap();
        map.lock()
            .unwrap()
            .remove(&format!("{}:__global:__:{}", user_id, key));
        Ok(())
    }
}
