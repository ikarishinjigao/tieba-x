#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Client error: {0}")]
    Client(#[from] reqwest::Error),

    #[error("Protobuf encode error: {0}")]
    Encode(#[from] proto::EncodeError),

    #[error("Protobuf decode error: {0}")]
    Decode(#[from] proto::DecodeError),

    #[error("Unexpected status code {}", reqwest::Response::status(.0))]
    UnexpectedStatusCode(reqwest::Response),

    #[error("API error: {0}")]
    Api(#[from] crate::api_error::ApiError),
}
