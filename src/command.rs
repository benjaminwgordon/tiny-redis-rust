pub mod command {
    use crate::resp::resp::RESP;
    use crate::DB::DB;

    #[derive(Debug)]
    pub enum COMMAND {
        GET { key: String },
        SET { key: String, value: String },
    }

    impl COMMAND {
        pub fn execute(&self, DB: &mut DB) -> String {
            match &self {
                COMMAND::GET { key } => {
                    let get_result = DB.get(key.to_string());
                    format!("{}", get_result)
                }
                COMMAND::SET { key, value } => {
                    let set_result = DB.set(key.to_string(), value.to_string());
                    format!("{}", set_result)
                }
            }
        }

        pub fn from_resp_array(arr: &RESP) -> Option<COMMAND> {
            if let RESP::ARRAY { value } = arr {
                let arguments = &value[1..];
                let cmd_or_err: Option<COMMAND> = match &value[0] {
                    RESP::BULK { value } => match value.as_ref() {
                        "GET" => {
                            if let RESP::BULK { value } = &arguments[0] {
                                Some(COMMAND::GET {
                                    key: value.to_string(),
                                })
                            } else {
                                println!("Argument 1 for GET is invalid");
                                None
                            }
                        }
                        "SET" => {
                            if let RESP::BULK { value } = &arguments[0] {
                                let key = value;
                                if let RESP::BULK { value } = &arguments[1] {
                                    Some(COMMAND::SET {
                                        key: key.to_string(),
                                        value: value.to_string(),
                                    })
                                } else {
                                    println!("Argument 2 for SET is invalid");
                                    None
                                }
                            } else {
                                println!("Argument 1 for SET is invalid");
                                None
                            }
                        }
                        _ => {
                            println!("Invalid Command: Valid commands are SET and GET");
                            return None;
                        }
                    },
                    _ => {
                        println!("Cannot parse commands from non-bulk values");
                        return None;
                    }
                };
                return cmd_or_err;
            } else {
                println!("cannot parse command from non-array RESP");
                return None;
            }
        }
    }
}
