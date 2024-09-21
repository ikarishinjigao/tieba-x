use crate::{
    api_request::ApiProtobufRequest,
    common::CommonRequest,
    enums::ThreadLoadType,
    protos::{
        personalized::{DataReq, PersonalizedReqIdl, PersonalizedResIdl},
        Error,
    },
    responses::GetThreadsPersonalizedResponse,
};

#[derive(Clone, Debug, uniffi::Record)]
pub struct GetThreadsPersonalizedRequest {
    pub load_type: ThreadLoadType,
    pub page_number: u32,
    pub page_size: u32,
}

impl Default for GetThreadsPersonalizedRequest {
    fn default() -> Self {
        Self {
            load_type: ThreadLoadType::Refresh,
            page_number: 1,
            page_size: 20,
        }
    }
}

impl ApiProtobufRequest for GetThreadsPersonalizedRequest {
    type ProtobufRequest = PersonalizedReqIdl;
    type ProtobufResponse = PersonalizedResIdl;
    type Response = GetThreadsPersonalizedResponse;

    fn path(&self) -> &str {
        "/c/f/excellent/personalized"
    }

    fn query(&self) -> &[(&str, &str)] {
        &[("cmd", "309264"), ("format", "protobuf")]
    }

    fn to_protobuf_request(&self) -> Self::ProtobufRequest {
        Self::ProtobufRequest {
            data: Some(DataReq {
                common: Some(CommonRequest::default().0),
                load_type: self.load_type as u32,
                pn: self.page_number,
                page_thread_count: self.page_size,
                q_type: 1,
                ..Default::default()
            }),
        }
    }

    fn to_error(&self, protobuf_response: &Self::ProtobufResponse) -> Error {
        protobuf_response.error.clone().unwrap_or_default()
    }

    fn to_response(&self, protobuf_response: &Self::ProtobufResponse) -> Self::Response {
        let _ = protobuf_response;
        Self::Response {}
    }
}
