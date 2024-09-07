use sha1::Digest;

use crate::HeliosHasher;

pub fn generate_cuid(android_id: &str) -> String {
    let value = format!("com.baidu{}", android_id);
    let md5_hex = format!("{:X}", md5::compute(value.as_bytes()));
    let helios_value = HeliosHasher::default().hash(md5_hex.as_bytes());
    format!("{}|V{}", md5_hex, helios_value)
}

pub fn generate_c3_aid(android_id: &str, uuid: &str) -> String {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{generate_c3_aid, generate_cuid};

    #[rstest]
    #[case("52ee55117d525049", "D4428B20707FDD378FCDD59CF1522CB2|V5OGHSON6")]
    fn test_generate_cuid(#[case] input: &str, #[case] expected: &str) {
        let result = generate_cuid(input);
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(("52ee55117d525049", "8cf4b37a-b859-4147-aa65-4f4bfd4b7c62"), "A00-XGCVTNFH2JY4QNBO3HYOMJZV7RHXFKON-4K7DJATL")]
    fn test_generate_c3_aid(#[case] input: (&str, &str), #[case] expected: &str) {
        let result = generate_c3_aid(input.0, input.1);
        assert_eq!(result, expected)
    }
}
