use crate::protos::CommonReq;

pub struct CommonRequest(pub CommonReq);

impl Default for CommonRequest {
    fn default() -> Self {
        Self(CommonReq {
            client_type: 2,
            client_version: crate::protos::TIEBA_CLIENT_VERSION.to_owned(),
            ..Default::default()
        })
    }
}
