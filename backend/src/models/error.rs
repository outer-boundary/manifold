use serde::Serialize;

type ErrorCode = u16;

// Model representing a user.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FieldError>>,
}

impl ErrorResponse {
    pub fn new(code: ErrorCode, message: String) -> Self {
        ErrorResponse {
            code,
            message,
            description: None,
            errors: None,
        }
    }

    pub fn description(&mut self, content: String) -> &Self {
        self.description = Some(content);
        self
    }

    pub fn field_errors(&mut self, errors: Vec<FieldError>) -> &Self {
        self.errors = Some(errors);
        self
    }
}

#[derive(Serialize)]
pub struct FieldError {
    pub code: ErrorCode,
    pub field: String,
    pub message: String,
}

impl FieldError {
    pub fn new(code: ErrorCode, field_name: String, message: String) -> Self {
        FieldError {
            code,
            field: field_name,
            message,
        }
    }
}
