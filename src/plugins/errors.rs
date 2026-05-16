use std::fmt::{Debug, Display};

use image::ImageError;

#[derive(Debug)]
pub enum Ra2Error {
    Io(std::io::Error),
    InvalidFormat { message: String },
    EncodeError { format: String, message: String },
}

impl Display for Ra2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ra2Error::Io(e) => {
                write!(f, "IO error: {}", e)
            }
            Ra2Error::InvalidFormat { message: e } => {
                write!(f, "Invalid file format: {}", e)
            }
            Ra2Error::EncodeError { format, message } => {
                write!(f, "Encode error: {}: {}", format, message)
            }
        }
    }
}

impl From<std::io::Error> for Ra2Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ImageError> for Ra2Error {
    fn from(error: ImageError) -> Self {
        Self::InvalidFormat {
            message: error.to_string(),
        }
    }
}
