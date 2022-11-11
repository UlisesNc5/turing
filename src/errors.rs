use std::io;

#[derive(Debug)]
pub enum ErrorCode{
    IO(io::Error),
    PreproError,
    ParseError,
    DuplicateDelta,
    Unhandled,
}

#[derive(Debug)]
pub struct Error{
    code : ErrorCode,
    extra: String,
}

impl Error{
    pub fn new_raw(code : ErrorCode) -> Self{
        return Error { code, extra: String::new()};
    }

    pub fn new_e_raw<S: AsRef<str>>(code : ErrorCode, extra : S) -> Self{
        Error {code, extra: extra.as_ref().to_string()}
    }

    pub fn new<V>(code : ErrorCode) -> Result<V, Self>{
        return Err(Error { code, extra: String::new()});
    }

    pub fn newe<V, S: AsRef<str>>(code : ErrorCode, extra : S) -> Result<V, Error>{
        return Err(Error::new_e_raw(code, extra));
    }
}
