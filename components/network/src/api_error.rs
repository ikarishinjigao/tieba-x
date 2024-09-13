#[derive(thiserror::Error, Debug, uniffi::Record)]
#[error("{:#?}", 0)]
pub struct ApiError {
    pub error_code: i32,
    pub error_message: String,
    pub user_message: String,
}
