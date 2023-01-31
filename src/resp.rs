pub mod resp {
    #[derive(Debug)]
    pub enum RESP {
        SIMPLE { value: String },
        ERROR { value: String },
        INTEGER { value: usize },
        BULK { value: String },
        ARRAY { value: Vec<RESP> },
        NULL,
    }

    pub fn parse_simple(bytes: &[u8], startIndex: usize) {}

    impl RESP {
        pub fn array_from_bytes(bytes: &[u8]) -> Result<RESP, String> {
            // RESP Array must start with "*"
            if bytes[0] != b'*' {
                println!(
                    "RESP array parse error: expected *, found: {:#?}",
                    &bytes[0]
                );
            }

            // get length of array in elements
            // TODO: this code can raise a lot of errors when passed invalid RESP
            //         that should be handled elegantly
            let i = 1;
            let end_size = RESP::read_until(bytes, b'\r', i);
            let array_len: usize = std::str::from_utf8(&bytes[i..end_size])
                .unwrap()
                .parse::<usize>()
                .unwrap();

            // i = end_size;

            // iterate over next arr_size elements, appending into array
            let array_contents = Vec::<RESP>::new();
            for _ in 0..array_len {
                array_contents.push();
            }

            return Ok(arr);
        }

        // reads bytes until specified byte char is seen.
        // returns the index where the byte character was found
        fn read_until(bytes: &[u8], target: u8, start_index: usize) -> usize {
            println!("{}", std::str::from_utf8(bytes).unwrap());
            let mut i = start_index;
            while i < bytes.len() && bytes[i] != target {
                //println!("comparing: {:#?} to {:#?}", bytes[i], target);
                i += 1
            }
            i
        }

        fn parse_next(bytes: &[u8], start_index: usize) {
            let end_size_index = RESP::read_until(bytes, b'\r', start_index);
            let len: usize = std::str::from_utf8(&bytes[start_index..end_size_index])
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let value = &bytes[start_index..start_index + len];

            // TODO: implement matcher for each possible line starting character
            let new_resp_token: RESP = match bytes[start_index] {
                b'+' => RESP::SIMPLE {
                    value: std::str::from_utf8(value).unwrap().to_string(),
                },
                b'$' => {
                    // write a new BULK
                    RESP::BULK {
                        value: std::str::from_utf8(value).unwrap().to_string(),
                    }
                }
                b':' => {
                    // write a new INTEGER
                    RESP::INTEGER {
                        value: std::str::from_utf8(value)
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    }
                }
                b'-' => {
                    // write a new ERROR
                    RESP::ERROR {
                        value: std::str::from_utf8(value).unwrap().to_string(),
                    }
                }
                _ => {}
            };
        }
    }
}
