use crc32fast::Hasher as Crc32;
use std::hash::Hasher;
use twox_hash::XxHash32;

const STATE_SIZE: usize = 5;
const BUFFER_SIZE: usize = 25;
const MASK: u64 = (1u64 << 32) - 1;

pub struct HeliosHasher {
    state: [u8; STATE_SIZE],
    buffer: [u8; BUFFER_SIZE],
}

impl Default for HeliosHasher {
    fn default() -> Self {
        Self {
            state: [0xFFu8; STATE_SIZE],
            buffer: [0xFFu8; BUFFER_SIZE],
        }
    }
}

impl HeliosHasher {
    #[inline(always)]
    pub fn hash(&mut self, bytes: &[u8]) -> String {
        let mut crc32_step1 = Crc32::default();
        crc32_step1.write(bytes);
        crc32_step1.write(&self.buffer[0..STATE_SIZE]);
        self.update_state_and_buffer(crc32_step1.finish(), 8, false, STATE_SIZE);

        let mut xx_hash_step2 = XxHash32::default();
        xx_hash_step2.write(bytes);
        xx_hash_step2.write(&self.buffer[0..STATE_SIZE * 2]);
        self.update_state_and_buffer(xx_hash_step2.finish(), 0, true, STATE_SIZE * 2);

        let mut xx_hash_step3 = xx_hash_step2;
        xx_hash_step3.write(&self.buffer[STATE_SIZE * 2..STATE_SIZE * 3]);
        self.update_state_and_buffer(xx_hash_step3.finish(), 1, true, STATE_SIZE * 3);

        let mut crc32_step4 = crc32_step1;
        crc32_step4.write(&self.buffer[STATE_SIZE..STATE_SIZE * 4]);
        self.update_state_and_buffer(crc32_step4.finish(), 7, true, STATE_SIZE * 4);

        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &self.state)
    }

    #[inline(always)]
    fn update_state_and_buffer(
        &mut self,
        hash_value: u64,
        start_bit: usize,
        use_xor: bool,
        buffer_offset: usize,
    ) {
        let mut state_value_padded = [0u8; 8];
        state_value_padded[..STATE_SIZE].copy_from_slice(&self.state);
        let state_value = u64::from_le_bytes(state_value_padded);
        let shifted_state = (state_value >> start_bit) & MASK;
        let new_value = if use_xor {
            shifted_state ^ hash_value
        } else {
            shifted_state & hash_value
        };
        let updated_state = (state_value & !(MASK << start_bit)) | (new_value << start_bit);
        self.state
            .copy_from_slice(&updated_state.to_le_bytes()[0..STATE_SIZE]);
        self.buffer[buffer_offset..buffer_offset + STATE_SIZE].copy_from_slice(&self.state);
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::HeliosHasher;

    #[rstest]
    #[case("", "SUJDFVPO")]
    #[case("f", "5ZFPL4DZ")]
    #[case("fo", "W55MKIXT")]
    #[case("foo", "2IX7Q5WQ")]
    #[case("foob", "CCI4A5NN")]
    #[case("fooba", "HXOZYFBH")]
    #[case("foobar", "EIXQJHFE")]
    fn test_helios_hash(#[case] input: &str, #[case] expected: &str) {
        let mut hasher = HeliosHasher::default();
        let result = hasher.hash(input.as_bytes());
        assert_eq!(result, expected)
    }
}
