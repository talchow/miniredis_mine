use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum ProcessError {
    #[error("not utf8 error")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("poison error {0}")]
    PoisonError(String),
    #[error("incomplete")]
    Incomplete,
}


// impl From<std::string::FromUtf8Error> for ProcessError {
//     fn from(value: FromUtf8Error) -> Self {
//         Self::FromUtf8Error(value)
//     }
// }    

// impl From<std::sync::PoisonError<String>> for ProcessError {
//     fn from(value: PoisonError<String>) -> Self {
//         Self::PoisonError(value)
//     }
// }

