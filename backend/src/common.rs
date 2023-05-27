use std::result::Result;

pub type MFError = Box<dyn std::error::Error>;

pub type MFResult<T> = Result<T, MFError>;
