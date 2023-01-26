enum RESP {
    SIMPLE { value: &str },
    ERROR { value: &str },
    INTEGER { value: &str },
    BULK { value: &str },
    ARRAY { value: &str },
    NULL,
}
