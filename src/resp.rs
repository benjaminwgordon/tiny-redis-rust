pub mod resp {
    #[derive(Debug)]
    pub enum RESP {
        BULK { value: String },
        ARRAY { value: Vec<RESP> },
    }

    impl RESP {
        pub fn array_from_bytes(bytes: &[u8]) -> Result<RESP, String> {
            // RESP Array must start with "*"
            if bytes[0] != b'*' {
                println!(
                    "RESP array parse error: expected *, found: {:#?}",
                    &bytes[0]
                );
            }

            // Create RESP ARRAY
            let mut arr = RESP::ARRAY {
                value: Vec::<RESP>::new(),
            };

            // get length of array in elements
            // TODO: this code can raise a lot of errors when passed invalid RESP
            //         that should be handled elegantly
            let mut cursor = 1;
            while bytes[cursor] != b'\r' {
                cursor += 1;
            }

            let array_len: usize = std::str::from_utf8(&bytes[1..cursor])
                .unwrap()
                .parse::<usize>()
                .unwrap();

            //println!("array length: {}", array_len);

            for _ in 0..array_len {
                // skip CRLF + $
                cursor += 3;

                // read content size
                // TODO: There should be a way to do this directly from the binary value
                // without parsing to string and then a number
                let mut len = 0;
                while bytes[cursor + len] != b'\r' {
                    len += 1;
                }
                let length = std::str::from_utf8(&bytes[cursor..cursor + len])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                // skip cursor to start of new content
                cursor += len + 2;

                let content = std::str::from_utf8(&bytes[cursor..cursor + length])
                    .unwrap()
                    .to_string();

                // TODO: currently only support bulk strings as the are the only valid input type for commands
                let bulk = RESP::BULK { value: content };
                if let RESP::ARRAY { ref mut value } = arr {
                    value.push(bulk);
                }
                cursor += length;
            }
            return Ok(arr);
        }
    }
}
