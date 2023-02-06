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
        InvalidRespArrayEncodingError,
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
        pub fn array_from_bytes(bytes: &[u8]) -> Result<RESP, RespParseError> {
            let (initial_cursor, array_len) = get_resp_array_length_from_bytes(&bytes)?;
            let mut cursor = initial_cursor;

            let mut arr = RESP::ARRAY {
                value: Vec::<RESP>::new(),
            };

            for _ in 0..array_len {
                let (updated_cursor, bulk) = get_next_bulk_string(cursor, &bytes)?;
                println!("bulk_string: {:?}", bulk);
                println!("new cursor position: {}", cursor);

                cursor = updated_cursor;

                if let RESP::ARRAY { ref mut value } = arr {
                    value.push(bulk);
                }
            }
            return Ok(arr);
        }
    }

    /**
     * reads a byte array representing a utf8 encoded RESP array, and gets the length of the array
     * Returns a tuple (x,y)
     * x: the new cursor position
     * y: the usize number of elements in this RESP array
     */
    fn get_resp_array_length_from_bytes(bytes: &[u8]) -> Result<(usize, usize), RespParseError> {
        // input commands must be RESP arrays starting with the "*" code
        if bytes[0] != b'*' {
            return Err(RespParseError::InvalidRespArrayEncodingError);
        }

        let mut cursor: usize = 1;
        let mut i = 1;
        while bytes[i + 1] != b'\r' {
            i += 1;
        }

        // parse the entire line up to the CLRF as a utf8 encoded string
        let array_length_as_string = std::str::from_utf8(&bytes[cursor..i + 1]);
        let array_length_as_string = match array_length_as_string {
            Err(err) => return Err(RespParseError::from_utf8_error(err)),
            Ok(val) => val,
        };

        // attempt to parse an integer value from the utf8 encoded string
        let array_length: Result<usize, ParseIntError> = array_length_as_string.parse();
        let array_length = match array_length {
            Err(err) => return Err(RespParseError::from_parse_int_error(err)),
            Ok(val) => val,
        };

        // skip the cursor to the end of the content
        cursor += i;

        // skip the CLRF and $
        cursor += 3;

        Ok((cursor, array_length))
    }

    /**
     * reads the line header to get the length of the next line in bytes, then parses that line as utf-8
     *
     * Returns a tuple, (x,y)
     * x: the new cursor position
     * y: the String contents of the encoded bulk_string
     */
    fn get_next_bulk_string(
        mut cursor: usize,
        bytes: &[u8],
    ) -> Result<(usize, RESP), RespParseError> {
        // count characters until next CLRF
        let mut i: usize = cursor;
        while bytes[i] != b'\r' {
            i += 1;
        }
        // scan the bytes up to the next CLRF and parse them as UTF-8
        let bulk_str_length_as_string = std::str::from_utf8(&bytes[cursor..i]);
        let bulk_str_length_as_string = match bulk_str_length_as_string {
            Err(err) => return Err(RespParseError::from_utf8_error(err)),
            Ok(val) => val,
        };

        // attempt to parse the UTF-8 encoded string as a number
        let bulk_str_length: Result<usize, ParseIntError> = bulk_str_length_as_string.parse();
        let bulk_str_length = match bulk_str_length {
            Err(err) => return Err(RespParseError::ParseIntError(err)),
            Ok(val) => val,
        };

        // move cursor to the end of the UTF-8 representing the byte-length of the bulk string
        cursor = i;

        // skip the RF and $
        cursor += 2;

        // attempt to parse the next n bytes after the cursor based on the run-length from the previous step
        let bulk_str_content = std::str::from_utf8(&bytes[cursor..cursor + bulk_str_length]);
        let bulk_str_content = match bulk_str_content {
            Err(err) => return Err(RespParseError::Utf8ParseError(err)),
            Ok(val) => val.to_string(),
        };

        // move cursor to the end of the bulk string content
        cursor = cursor + bulk_str_length;

        // skip CLRF and $
        cursor += 3;

        // construct an RESP BULK to hold this bulk string
        let resp_bulk_str = RESP::BULK {
            value: bulk_str_content,
        };

        Ok((cursor, resp_bulk_str))
    }
}
