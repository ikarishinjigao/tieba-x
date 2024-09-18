use crate::protos::Error;
use prost::Message;
use reqwest::{
    header::{HeaderMap, CACHE_CONTROL, CONNECTION, USER_AGENT},
    Method,
};

pub trait ApiProtobufRequest: Send + Sync
where
    Self::ProtobufRequest: Message + Default,
    Self::ProtobufResponse: Message + Default,
{
    type ProtobufRequest;
    type ProtobufResponse;
    type Response;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> &str;
    fn query(&self) -> &[(&str, &str)];

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("x_bd_data_type", "protobuf".parse().unwrap());
        headers.insert(
            USER_AGENT,
            format!("tieba/{}", crate::protos::TIEBA_CLIENT_VERSION)
                .parse()
                .unwrap(),
        );
        headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
        headers.insert(CONNECTION, "keep-alive".parse().unwrap());
        headers
    }

    fn to_protobuf_request(&self) -> Self::ProtobufRequest;
    fn to_error(&self, protobuf_response: &Self::ProtobufResponse) -> Error;
    fn to_response(&self, protobuf_response: &Self::ProtobufResponse) -> Self::Response;
}
