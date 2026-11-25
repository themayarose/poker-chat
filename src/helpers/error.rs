use std::fmt;
use std::error;

use tokio_tungstenite::tungstenite;

use crate::app::Message;

#[derive(Debug, Clone, PartialEq)]
pub struct AppError {
    pub title: String,
    pub message: String,
    pub action: AppErrorAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppErrorAction {
    Log,
    Display,
    Crash,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.title, self.message)
    }
}

impl error::Error for AppError {}

impl From<AppError> for Message {
    fn from(err: AppError) -> Self {
        Message::Error(err)
    }
}

impl From<Box<dyn error::Error>> for Message {
    fn from(err: Box<dyn error::Error>) -> Self {
        <Box<dyn error::Error> as Into<AppError>>::into(err).into()
    }
}

impl From<Box<dyn error::Error>> for AppError {
    fn from(err: Box<dyn error::Error>) -> Self {
        match err.downcast::<tungstenite::Error>() {
            Ok(normal) if matches!(*normal, tungstenite::Error::ConnectionClosed) => AppError::log("Client", "Disconnected normally"),
            Ok(tng_err) => AppError::err("Error", format!("{tng_err}")),
            Err(other_err) => AppError::err("Error", format!("{other_err}")),
        }
    }
}

impl AppError {
    pub fn err(title: impl Into<String>, message: impl Into<String>) -> AppError {
        AppError {
            title: title.into().clone(),
            message: message.into().clone(),
            action: AppErrorAction::Display,
        }
    }

    pub fn log(title: impl Into<String>, message: impl Into<String>) -> AppError {
        AppError {
            title: title.into().clone(),
            message: message.into().clone(),
            action: AppErrorAction::Log,
        }
    }
}
