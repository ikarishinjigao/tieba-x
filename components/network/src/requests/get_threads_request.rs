use crate::{
    api_request::ApiProtobufRequest, common::CommonRequest, enums::ThreadSortType,
    responses::GetThreadsResponse,
};
use proto::frs_page::{DataReq, FrsPageReqIdl, FrsPageResIdl};

#[derive(Debug)]
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

    fn to_error(&self, protobuf_response: Self::ProtobufResponse) -> proto::Error {
        protobuf_response.error.unwrap_or_default()
    }

    fn to_response(&self, protobuf_response: Self::ProtobufResponse) -> Self::Response {
        let data = protobuf_response.data.unwrap_or_default();
        let forum = data.forum.unwrap_or_default();
        Self::Response {
            forum_id: forum.id,
            forum_name: forum.name,
        }
    }
}
