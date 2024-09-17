#[derive(thiserror::Error, Debug, uniffi::Error)]
pub enum Error {
    #[error("Client error: {error_message}")]
    Client { error_message: String },

    #[error("Protobuf encode error: {error_message}")]
    Encode { error_message: String },

    #[error("Protobuf decode error: {error_message}")]
    Decode { error_message: String },

    #[error("Unexpected status code: {status_code}")]
    UnexpectedStatusCode { status_code: u16 },

    #[error("API error: {api_error}")]
    Api {
        #[from]
        api_error: crate::api_error::ApiError,
    },
}
