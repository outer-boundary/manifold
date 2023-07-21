use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

type ErrorCode = u16;

// Model representing a user.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FieldError>>,
}

impl ErrorResponse {
    pub fn new<T>(code: ErrorCode, message: T) -> Self
    where
        T: ToString,
    {
        ErrorResponse {
            code,
            message: message.to_string(),
            description: None,
            errors: None,
        }
    }

    pub fn description<T>(&self, content: T) -> Self
    where
        T: ToString,
    {
        ErrorResponse {
            code: self.code,
            message: self.message.clone(),
            description: Some(content.to_string()),
            errors: self.errors.clone(),
        }
    }

    pub fn field_errors<T>(&self, errors: Vec<FieldError>) -> Self {
        ErrorResponse {
            code: self.code,
            message: self.message.clone(),
            description: self.description.clone(),
            errors: Some(errors),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldError {
    pub code: ErrorCode,
    pub field: String,
    pub message: String,
}

impl FieldError {
    pub fn new<T>(code: ErrorCode, field_name: String, message: T) -> Self
    where
        T: ToString,
    {
        FieldError {
            code,
            field: field_name,
            message: message.to_string(),
        }
    }
}

macro_rules! extractor_error {
    ($(($variant:ident, $status_code:literal)),* $(,)?) => {
        #[derive(Debug)]
        pub enum ExtractorError {
            $(
                $variant(ErrorResponse),
            )*
        }

        impl std::fmt::Display for ExtractorError {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        ExtractorError::$variant(err) => write!(f, "{} ({}): {:?}", stringify!($variant), stringify!($status_code), err),
                    )*
                }
            }
        }

        impl ResponseError for ExtractorError {
            fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
                match self {
                    $(
                        ExtractorError::$variant(msg) => HttpResponse::$variant().json(msg),
                    )*
                }
            }
        }
    }
}

extractor_error! {
    (BadRequest, 400),
    (Unauthorized, 401),
    (Forbidden, 403),
    (InternalServerError, 500),
}
