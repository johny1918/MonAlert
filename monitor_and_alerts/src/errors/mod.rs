use std::fmt::Display;


#[derive(Debug)]
pub enum CustomError {
    Error(String)
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::Error(msg) => write!(f, "{}", msg),
        }
    }
}
