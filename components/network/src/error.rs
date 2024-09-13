#[derive(thiserror::Error, Debug, uniffi::Error)]
pub enum Error {
    #[error("Client error: {message}")]
    Client { message: String },

    #[error("Protobuf encode error: {message}")]
    Encode { message: String },

    #[error("Protobuf decode error: {message}")]
    Decode { message: String },

    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(u16),

    #[error("API error: {0}")]
    Api(#[from] crate::api_error::ApiError),
}
