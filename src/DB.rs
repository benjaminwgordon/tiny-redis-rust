use std::collections::HashMap;

#[derive(Debug)]
pub struct DB {
    db: HashMap<String, String>,
}

impl DB {
    pub fn new() -> DB {
        DB { db: HashMap::new() }
    }

    pub fn set(&mut self, key: String, val: String) -> String {
        self.db.insert(key, val);
        "OK".to_string()
    }

    pub fn get(&self, key: String) -> String {
        if !self.db.contains_key(&key) {
            return "Key not found".to_string();
        } else {
            return self.db.get(&key).unwrap().to_string();
        }
    }
}
