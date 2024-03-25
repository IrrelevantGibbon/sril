#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(i32),
    Unit,
}

use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Unit => write!(f, "Unit"),
        }
    }
}
