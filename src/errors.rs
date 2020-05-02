use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct QueryError {
    pub description: String,
}


impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<QueryError {}>", self.description)
    }
}


impl Error for QueryError {}
