// (c) 2025,26 Konstantin Adamov, licensed under MIT

use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct Eui64Generator;

/// Implementation of BaseGenerator for generating EUI-64 identifiers.
impl BaseGenerator for Eui64Generator {
    fn name(&self) -> &str {
        "EUI-64 Identifier"
    }
    fn digist_are_editable(&self) -> bool {
        false
    }
    fn default_digits(&self) -> usize {
        16
    }

    fn generate(&self, lines: usize, _digits: usize, upper_case: bool) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();

        for _ in 0..lines {
            let octets: Vec<String> = (0..8)
                .map(|_| {
                    let byte = rng.gen_range(0u8..=255u8);
                    if upper_case {
                        format!("{:02X}", byte)
                    } else {
                        format!("{:02x}", byte)
                    }
                })
                .collect();
            result.push_str(&octets.join(":"));
            result.push('\n');
        }

        result
    }
}
