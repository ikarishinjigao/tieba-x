use crate::api_error::ApiError;
use crate::{ApiProtobufRequest, Error};
use proto::Message;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use reqwest::{Client, Url};

#[derive(Debug)]
pub struct ApiClient {
    client: Client,
}

impl Default for ApiClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl ApiClient {
    pub async fn request<ApiRequest>(
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
                .map_err(|e| Error::Encode(e))?;

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
            .map_err(|e| Error::Client(e))?;

        if !response.status().is_success() {
            return Err(Error::UnexpectedStatusCode(response));
        }

        let protobuf_response = {
            let body = response.bytes().await.map_err(|e| Error::Client(e))?;
            ApiRequest::ProtobufResponse::decode(body).map_err(|e| Error::Decode(e))?
        };
        let protobuf_error = api_request.to_error(protobuf_response.clone());
        let api_error = ApiError {
            error_code: protobuf_error.errorno,
            error_message: protobuf_error.errmsg,
            user_message: protobuf_error.usermsg,
        };
        if api_error.error_code != 0 {
            Err(Error::Api(api_error))
        } else {
            Ok(api_request.to_response(protobuf_response))
        }
    }
}
