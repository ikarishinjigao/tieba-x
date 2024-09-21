#[repr(u32)]
#[derive(Clone, Copy, Debug, uniffi::Enum)]
pub enum ThreadLoadType {
    Refresh = 1,
    LoadMore = 2,
}
