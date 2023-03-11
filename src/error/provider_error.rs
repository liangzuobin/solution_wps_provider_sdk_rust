use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use strum::Display;

#[derive(Debug, Display)]
pub enum ProviderError {
    InvalidUserToken,
    PermissionDenied,
    FileNotExist,
    InvalidArgument(String),
    SpaceFull,
    CustomError(StatusCode, String),
    FileNameConflict,
    VersionNotExist,
    UserNotExist,
    FileUploadNotComplete,
    ServerError(String),
}

impl ProviderError {
    fn status_code_message(&self) -> (StatusCode, i32, String) {
        let msg = self.to_string();
        let status_and_code = match self {
            ProviderError::InvalidUserToken => (StatusCode::FORBIDDEN, 40002, msg),
            ProviderError::PermissionDenied => (StatusCode::FORBIDDEN, 40003, msg),
            ProviderError::FileNotExist => (StatusCode::BAD_REQUEST, 40004, msg),
            ProviderError::InvalidArgument(_) => (StatusCode::BAD_REQUEST, 40005, msg),
            ProviderError::SpaceFull => (StatusCode::FORBIDDEN, 40006),
            ProviderError::CustomError(status_code, _) => (*status_code, 40007),
            ProviderError::FileNameConflict => (StatusCode::FORBIDDEN, 40008),
            ProviderError::VersionNotExist => (StatusCode::BAD_REQUEST, 40009),
            ProviderError::UserNotExist => (StatusCode::BAD_REQUEST, 40010),
            ProviderError::FileUploadNotComplete => (StatusCode::FORBIDDEN, 41001),
            ProviderError::ServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
        }
    }
}

impl ResponseError for ProviderError {
    fn status_code(&self) -> StatusCode {
        return self.status_and_error_code().0;
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}