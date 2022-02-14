pub trait Storage {
    fn create(&self, user_id: &str, key: String, value: &[u8]) -> Result<(), String>;

    fn read(&self, user_id: &str, key: &str) -> Result<Option<Vec<u8>>, String>;

    fn read_key_list(&self, user_id: &str) -> Result<Vec<String>, String>;

    fn read_batch(&self, user_id: &str, keys: Vec<String>) -> Result<Vec<Option<Vec<u8>>>, String>;

    fn update(&self, user_id: &str, key: &str, value: &[u8]) -> Result<(), String>;

    fn delete(&self, user_id: &str, key: &str) -> Result<(), String>;
}
