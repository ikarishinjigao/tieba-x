use crate::{
    api_request::ApiProtobufRequest,
    common::CommonRequest,
    enums::ThreadSortType,
    protos::frs_page::{DataReq, FrsPageReqIdl, FrsPageResIdl},
    protos::Error,
    responses::GetThreadsResponse,
};

#[derive(Clone, Debug, uniffi::Record)]
pub struct GetThreadsRequest {
    pub forum_name: String,
    pub page_number: i32,
    pub page_size: i32,
    pub sort_type: ThreadSortType,
    pub is_high_quality_thread: bool,
}

impl Default for GetThreadsRequest {
    fn default() -> Self {
        Self {
            forum_name: "".to_owned(),
            page_number: 1,
            page_size: 5,
            sort_type: ThreadSortType::CreateTime,
            is_high_quality_thread: false,
        }
    }
}

impl ApiProtobufRequest for GetThreadsRequest {
    type ProtobufRequest = FrsPageReqIdl;
    type ProtobufResponse = FrsPageResIdl;
    type Response = GetThreadsResponse;

    fn path(&self) -> &str {
        "/c/f/frs/page"
    }

    fn query(&self) -> &[(&str, &str)] {
        &[("cmd", "301001"), ("format", "protobuf")]
    }

    fn to_protobuf_request(&self) -> Self::ProtobufRequest {
        Self::ProtobufRequest {
            data: Some(DataReq {
                common: Some(CommonRequest::default().0),
                kw: self.forum_name.clone(),
                pn: self.page_number,
                rn: self.page_size,
                rn_need: self.page_size,
                sort_type: self.sort_type as i32,
                is_good: self.is_high_quality_thread.into(),
                ..Default::default()
            }),
        }
    }

    fn to_error(&self, protobuf_response: &Self::ProtobufResponse) -> Error {
        protobuf_response.error.clone().unwrap_or_default()
    }

    fn to_response(&self, protobuf_response: &Self::ProtobufResponse) -> Self::Response {
        let forum = protobuf_response
            .data
            .as_ref()
            .and_then(|x| x.forum.as_ref());
        Self::Response {
            forum_id: forum.and_then(|x| Some(x.id)).unwrap_or_default(),
            forum_name: forum.and_then(|x| Some(x.name.clone())).unwrap_or_default(),
        }
    }
}
