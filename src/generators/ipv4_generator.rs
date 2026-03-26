// (c) 2025,26 Konstantin Adamov, licensed under MIT

use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct IPv4Generator;

/// Implementation of BaseGenerator for generating random IPv4 addresses.
impl BaseGenerator for IPv4Generator {
    fn name(&self) -> &str {
        "IPv4 Address"
    }
    fn digist_are_editable(&self) -> bool {
        false
    }
    fn default_digits(&self) -> usize {
        8
    }

    fn generate(&self, lines: usize, _digits: usize, _upper_case: bool) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();

        for _ in 0..lines {
            let octets: [u8; 4] = [
                rng.gen_range(0u8..=255u8),
                rng.gen_range(0u8..=255u8),
                rng.gen_range(0u8..=255u8),
                rng.gen_range(0u8..=255u8),
            ];
            result.push_str(&format!(
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            ));
            result.push('\n');
        }

        result
    }
}
