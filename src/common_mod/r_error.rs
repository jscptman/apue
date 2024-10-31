use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct MsgError<T:Debug> {
    message: T,
}

impl<T: Debug> MsgError<T> {
    pub fn from_message(message: T) -> Self {
        Self { message }
    }
}

impl<T:Debug> Display for MsgError<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}
impl<T> Error for MsgError<T> where T: Display + Debug {}
