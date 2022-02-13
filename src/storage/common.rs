pub trait Storage {
    fn create(&self, user_id: String, key: String, value: &[u8]) -> Result<(), String>;

    fn read(&self, user_id: String, key: &String) -> Result<Option<Vec<u8>>, String>;

    fn update(&self, user_id: String, key: String, value: &[u8]) -> Result<(), String>;

    fn delete(&self, user_id: String, key: String) -> Result<(), String>;
}
