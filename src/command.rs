#[derive(Debug)]
pub enum COMMAND {
    GET { key: String },
    SET { key: String, value: String },
}

impl COMMAND {
    pub fn execute(&self, key: &str, value: Option<&str>) {
        match &self {
            COMMAND::GET { key } => {
                println!("Executing GET on key: {}", key)
            }
            COMMAND::SET { key, value } => {
                println!("Executing SET on key:val pair {}:{}", key, value)
            }
        }
    }

    // converts byte string into COMMAND
    pub fn from_bytes(bytes: usize) -> COMMAND {
        println!("{:#?}", bytes);
        return COMMAND::GET {
            key: "TEST".to_string(),
        };
    }
}
