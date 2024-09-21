use crate::{
    api_error::ApiError,
    api_request::ApiProtobufRequest,
    error::Error,
    requests::{GetThreadsPersonalizedRequest, GetThreadsRequest},
    responses::{GetThreadsPersonalizedResponse, GetThreadsResponse},
};
use prost::Message;
use reqwest::{
    multipart::{Form, Part},
    Client, Response, StatusCode, Url,
};

#[derive(Debug, uniffi::Object)]
pub struct ApiClient {
    client: Client,
}

#[uniffi::export]
impl ApiClient {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            client: Client::builder().gzip(true).build().unwrap(),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl ApiClient {
    pub async fn get_threads(
        &self,
        request: GetThreadsRequest,
    ) -> Result<GetThreadsResponse, Error> {
        self.request(request).await
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl ApiClient {
    pub async fn get_threads_personalized(
        &self,
        request: GetThreadsPersonalizedRequest,
    ) -> Result<GetThreadsPersonalizedResponse, Error> {
        self.request(request).await
    }
}

impl ApiClient {
    async fn request<ApiRequest>(
        &self,
        api_request: ApiRequest,
    ) -> Result<ApiRequest::Response, Error>
    where
        ApiRequest: ApiProtobufRequest,
    {
        let url = {
            let mut url = Url::parse("https://tiebac.baidu.com").unwrap();
            url.set_path(api_request.path());
            url
        };
        let form = {
            let protobuf_request = api_request.to_protobuf_request();
            let mut protobuf_request_buf = vec![];
            protobuf_request
                .encode(&mut protobuf_request_buf)
                .map_err(|e| Error::Encode {
                    error_message: e.to_string(),
                })?;
            let part = Part::bytes(protobuf_request_buf).file_name("data");
            Form::new().part("data", part)
        };

        let response = self
            .client
            .post(url)
            .query(api_request.query())
            .headers(api_request.headers())
            .multipart(form)
            .send()
            .await
            .map_err(|e| Error::Client {
                error_message: e.to_string(),
            })?;

        match response.status() {
            StatusCode::OK => {
                let protobuf_response = {
                    let body = response.bytes().await.map_err(|e| Error::Client {
                        error_message: e.to_string(),
                    })?;
                    stacker::grow(8 * 1024 * 1024, || {
                        ApiRequest::ProtobufResponse::decode(body).map_err(|e| Error::Decode {
                            error_message: e.to_string(),
                        })
                    })?
                };
                let protobuf_error = api_request.to_error(&protobuf_response);
                let api_error = ApiError {
                    error_code: protobuf_error.errorno,
                    error_message: protobuf_error.errmsg,
                    user_message: protobuf_error.usermsg,
                };
                if api_error.error_code != 0 {
                    Err(Error::Api { api_error })
                } else {
                    Ok(api_request.to_response(&protobuf_response))
                }
            }
            _ => Err(Error::UnexpectedStatusCode {
                status_code: Response::status(&response).into(),
            }),
        }
    }
}
