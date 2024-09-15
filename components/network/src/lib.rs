uniffi::setup_scaffolding!("TiebaxNetwork");

mod api_client;
mod api_error;
mod api_request;
mod common;
mod enums;
mod error;
mod requests;
mod responses;

use api_request::ApiProtobufRequest;

pub use api_client::ApiClient;
pub use error::Error;
pub use requests::GetThreadsRequest;
pub use responses::GetThreadsResponse;

#[uniffi::export(async_runtime = "tokio")]
impl ApiClient {
    async fn get_threads(&self, request: GetThreadsRequest) -> Result<GetThreadsResponse, Error> {
        self.request(request).await
    }
}
