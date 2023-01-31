pub mod command {

    #[derive(Debug)]
    pub enum COMMAND {
        GET { key: String },
        SET { key: String, value: String },
    }

    impl COMMAND {
        pub fn execute(&self) {
            match &self {
                COMMAND::GET { key } => {
                    println!("Executing GET on key: {}", key)
                }
                COMMAND::SET { key, value } => {
                    println!("Executing SET on key:val pair {}:{}", key, value)
                }
            }
        }

        // // converts byte string into COMMAND
        // pub fn from_RESP(resp: crate::resp::resp::RESP) -> Result<COMMAND, Utf8Error> {
        //     //let parse_command =
        // }
    }
}
