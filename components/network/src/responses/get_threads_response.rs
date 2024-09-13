#[derive(Default, Debug, uniffi::Record)]
pub struct GetThreadsResponse {
    pub forum_id: i64,
    pub forum_name: String,
}
