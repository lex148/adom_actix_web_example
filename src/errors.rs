use actix_web::{error::ResponseError, HttpResponse};
use log::error;
use std::convert::From;
use std::net::AddrParseError;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Serialize, Debug)]
pub struct ValidationError {
    pub field: Option<String>,
    pub message: String,
}

impl ValidationError {
    fn display(&self) -> String {
        match self.field {
            Some(ref f) => format!("{}: {}", f, self.message),
            None => format!("{}", self.message),
        }
    }
}

pub fn validation_error(msg: &str, field: Option<&str>) -> ServiceError {
    let err = ValidationError {
        field: field.map(|f| f.to_string()),
        message: msg.to_string(),
    };
    ServiceError::ValidationErrors(vec![err])
}

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
    ValidationErrors(Vec<ValidationError>),
    BadRequest(String),
    NotFound,
    Unauthorized,
    BadToken,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(ServiceError)")
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound => HttpResponse::NotFound().json("NotFound"),
            ServiceError::BadToken => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::ValidationErrors(ref err) => {
                let messages: Vec<String> = err.iter().map(|v| v.display()).collect();
                HttpResponse::BadRequest().json(messages)
            }
        }
    }
}

impl From<serde_json::error::Error> for ServiceError {
    fn from(inner: serde_json::error::Error) -> ServiceError {
        error!("{:?}", inner);
        ServiceError::InternalServerError
    }
}

impl From<std::num::ParseIntError> for ServiceError {
    fn from(inner: std::num::ParseIntError) -> ServiceError {
        error!("{:?}", inner);
        ServiceError::InternalServerError
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(inner: std::io::Error) -> ServiceError {
        error!("{:?}", inner);
        ServiceError::InternalServerError
    }
}

impl From<tokio_postgres::error::Error> for ServiceError {
    fn from(inner: tokio_postgres::error::Error) -> ServiceError {
        error!("{:?}", inner);
        ServiceError::InternalServerError
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for ServiceError {
    fn from(inner: bb8::RunError<tokio_postgres::error::Error>) -> ServiceError {
        error!("{:?}", inner);
        ServiceError::InternalServerError
    }
}

impl From<AddrParseError> for ServiceError {
    fn from(_inner: AddrParseError) -> ServiceError {
        ServiceError::BadRequest("invalid IP address".to_owned())
    }
}

impl From<std::string::FromUtf8Error> for ServiceError {
    fn from(inner: std::string::FromUtf8Error) -> ServiceError {
        error!("{:?}", inner);
        ServiceError::InternalServerError
    }
}
