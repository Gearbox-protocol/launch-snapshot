use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AirdropError {
    NetError(String),
}

impl Display for AirdropError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AirdropError::NetError(str) => {
                write!(f, "{}", str)
            }
        }
    }
}
