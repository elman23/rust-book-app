use mobc_postgres::tokio_postgres;
use serde_derive::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{filters::ws::Message, http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error getting connection from DB pool: {0}")]
    DBPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("Error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("Error creating table: {0}")]
    DBInitError(tokio_postgres::Error),
    #[error("Error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
    #[error("generic error")]
    GenericError,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid body";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::DBQueryError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not execute request";
            }
            Error::WrongCredentialsError => {
                code = StatusCode::FORBIDDEN;
                message = "Forbidden";
            }
            Error::NoPermissionError => {
                code = StatusCode::UNAUTHORIZED;
                message = "Unauthorized";
            }
            Error::JWTTokenError => {
                code = StatusCode::UNAUTHORIZED;
                message = "Unauthorized"
            }
            Error::JWTTokenCreationError => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
            Error::GenericError => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Generic Error";
            }
            _ => {
                eprintln!("Unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal server error";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method not allowed";
    } else {
        eprintln!("Unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
        status: code.to_string(),
    });

    Ok(warp::reply::with_status(json, code))
}
