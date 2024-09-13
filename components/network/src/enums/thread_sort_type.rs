#[repr(i32)]
#[derive(Clone, Copy, Debug, uniffi::Enum)]
pub enum ThreadSortType {
    CreateTime = 1,
    FollowedOnly = 2,
    Popularity = 3,
    ReplyTime = 5,
}
