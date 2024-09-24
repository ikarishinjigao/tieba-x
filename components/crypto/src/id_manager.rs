use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha1::Digest;
use uuid::Uuid;

use crate::HeliosHasher;

#[derive(uniffi::Object)]
pub struct IdManager {}

#[uniffi::export]
impl IdManager {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }
}

#[uniffi::export]
impl IdManager {
    pub fn generate_uuid(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn generate_android_id(&self, seed: u64) -> String {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let random_bytes: [u8; 8] = rng.gen();
        let random_long = i64::from_le_bytes(random_bytes);
        format!("{:016x}", random_long.abs())
    }

    pub fn generate_cuid(&self, android_id: &str) -> String {
        let value = format!("com.baidu{}", android_id);
        let md5_hex = format!("{:X}", md5::compute(value.as_bytes()));
        let helios_value = HeliosHasher::default().hash(md5_hex.as_bytes());
        format!("{}|V{}", md5_hex, helios_value)
    }

    pub fn generate_c3_aid(&self, android_id: &str, uuid: &str) -> String {
        let value = format!("com.helios{}{}", android_id, uuid);
        let sha1_value = {
            let mut sha1_hasher = sha1::Sha1::default();
            sha1_hasher.update(value.as_bytes());
            sha1_hasher.finalize()
        };
        let sha1_base32 = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &sha1_value);
        let prefix = format!("A00-{}-", sha1_base32);
        let helios_value = HeliosHasher::default().hash(prefix.as_bytes());
        format!("{}{}", prefix, helios_value)
    }
}

#[cfg(test)]
mod tests {
    use super::IdManager;
    use rstest::rstest;

    #[rstest]
    fn test_generate_uuid() {
        let result = IdManager::new().generate_uuid();
        assert_eq!(result.len(), 36);
        assert_eq!(result.chars().filter(|&c| c == '-').count(), 4);
        assert!(result.replace("-", "").chars().all(|c| c.is_digit(16)));
    }

    #[rstest]
    #[case(28, "3bf49d47909c5f4b")]
    fn test_generate_android_id(#[case] input: u64, #[case] expected: &str) {
        let result = IdManager::new().generate_android_id(input);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 16);
        assert!(result.chars().all(|c| c.is_digit(16)));
        assert!(result
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[rstest]
    #[case("3bf49d47909c5f4b", "BE5BECFE144DC736616F9A2F2BFA5DC0|VNRT6J3XJ")]
    fn test_generate_cuid(#[case] input: &str, #[case] expected: &str) {
        let result = IdManager::new().generate_cuid(input);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 42);
    }

    #[rstest]
    #[case(("3bf49d47909c5f4b", "8cf4b37a-b859-4147-aa65-4f4bfd4b7c62"), "A00-Q6DBHSMBXLXEXWJG22EMOPMKVKCQSM2C-4GLJYLP7")]
    fn test_generate_c3_aid(#[case] input: (&str, &str), #[case] expected: &str) {
        let result = IdManager::new().generate_c3_aid(input.0, input.1);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 45);
    }
}
