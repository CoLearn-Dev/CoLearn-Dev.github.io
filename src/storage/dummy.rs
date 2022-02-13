use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::OnceCell;

static MAP: OnceCell<Mutex<HashMap<String, Vec<u8>>>> = OnceCell::new();

pub struct DummyStorage;

impl DummyStorage {
    pub fn new() -> Self {
        MAP.set(Mutex::new(HashMap::new()));
        Self {}
    }
}

impl crate::storage::common::Storage for DummyStorage {
    fn create(&self, user_id: String, key: String, value: &[u8]) -> Result<(), String> {
        MAP.get().unwrap().lock().unwrap().insert(key, value.to_vec());
        Ok(())
    }

    fn read(&self, user_id: String, key: &String) -> Result<Option<Vec<u8>>, String> {
        let map = MAP.get().unwrap().lock().unwrap();
        let res = map.get(key);
        Ok(match res {
            None => None,
            Some(s) => Some(s.clone())
        })
    }

    fn update(&self, user_id: String, key: String, value: &[u8]) -> Result<(), String> {
        todo!()
    }

    fn delete(&self, user_id: String, key: String) -> Result<(), String> {
        todo!()
    }
}
