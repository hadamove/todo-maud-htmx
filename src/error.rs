use serde::Serialize;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum ApiError {
    #[error("Internal server error")]
    InternalServerError,
    #[error("Not found")]
    NotFound,
    #[error("Bad request")]
    BadRequest,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(_) => Self::BadRequest,
            _ => Self::InternalServerError,
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            Self::BadRequest => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
