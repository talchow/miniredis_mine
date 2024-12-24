use core::fmt;
use std::error::Error;
use std::str::FromStr;
use std::string::FromUtf8Error;
use std::sync::PoisonError;

#[derive(Debug)]
pub enum ProcessError {

    FromUtf8Error(FromUtf8Error),
    PoisonError(PoisonError<String>),
    Incomplete,
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           Self::FromUtf8Error(u) => write!(f, "{}",u),
           Self::PoisonError(p) => write!(f, "{}",p),
           Self::Incomplete => "the stream closed too early".fmt(f),
       }
    }
}

impl Error for ProcessError {}

impl From<std::string::FromUtf8Error> for ProcessError {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}    

impl From<std::sync::PoisonError<String>> for ProcessError {
    fn from(value: PoisonError<String>) -> Self {
        Self::PoisonError(value)
    }
}


    