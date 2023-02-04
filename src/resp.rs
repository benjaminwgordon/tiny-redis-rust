pub mod resp {
    use std::num::ParseIntError;
    use std::str::Utf8Error;

    #[derive(Debug)]
    pub enum RESP {
        BULK { value: String },
        ARRAY { value: Vec<RESP> },
    }

    #[derive(Debug)]
    pub enum RespParseError {
        Utf8ParseError(Utf8Error),
        ParseIntError(ParseIntError),
    }

    impl RespParseError {
        fn from_utf8_error(err: Utf8Error) -> RespParseError {
            RespParseError::Utf8ParseError(err)
        }

        fn from_parse_int_error(err: ParseIntError) -> RespParseError {
            RespParseError::ParseIntError(err)
        }
    }

    impl RESP {
        pub fn array_len_from_byte_slice(
            bytes: &[u8],
            start: usize,
            end: usize,
        ) -> Result<usize, RespParseError> {
            let array_len_as_utf8 = std::str::from_utf8(&bytes[start..end]);
            let array_len_as_str = match array_len_as_utf8 {
                Err(err) => return Err(RespParseError::from_utf8_error(err)),
                Ok(val) => val,
            };
            let array_len = array_len_as_str.parse::<usize>();
            match array_len {
                Err(err) => {
                    println!("{}", err);
                    return Err(RespParseError::from_parse_int_error(err));
                }
                Ok(val) => return Ok(val),
            }
        }

        pub fn token_from_byte_slice(
            bytes: &[u8],
            start: usize,
            end: usize,
        ) -> Result<String, RespParseError> {
            let content = std::str::from_utf8(&bytes[start..end]);

            match content {
                Err(err) => return Err(RespParseError::from_utf8_error(err)),
                Ok(val) => return Ok(val.to_string()),
            }
        }

        pub fn get_token_byte_length(cursor: usize, bytes: &[u8]) -> usize {
            let mut len: usize = 0;
            let mut i: usize = cursor;
            while bytes[i] != b'\r' {
                len += 1;
                i += 1;
            }
            return len;
        }

        pub fn array_from_bytes(bytes: &[u8]) -> Result<RESP, RespParseError> {
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
            let mut cursor = 1;
            let arr_len = RESP::get_token_byte_length(cursor, &bytes);

            let array_len = RESP::array_len_from_byte_slice(&bytes, 1, 1 + arr_len)?;

            cursor += 1;

            for _ in 0..array_len {
                // skip CRLF + $
                cursor += 3;

                // read content size
                let token_length = RESP::get_token_byte_length(cursor, &bytes);

                println!("token width: {}", token_length);

                let length = RESP::get_token_byte_length(&bytes, cursor, cursor + token_length)?;

                // skip cursor to start of new content
                cursor += token_length + 2;

                let content = RESP::token_from_byte_slice(&bytes, cursor, cursor + token_length)?;

                println!("content: {}", content);

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
