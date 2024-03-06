use serde::Serialize;


#[derive(Debug, Serialize)]
pub enum CustomError {
    NotFound(String),
    InternalError(String),
    BadRequest(String)
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: CustomError
}