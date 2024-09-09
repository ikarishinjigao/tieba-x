pub struct CommonRequest(pub proto::CommonReq);

impl Default for CommonRequest {
    fn default() -> Self {
        Self(proto::CommonReq {
            client_type: 2,
            client_version: proto::TIEBA_CLIENT_VERSION.to_owned(),
            ..Default::default()
        })
    }
}
