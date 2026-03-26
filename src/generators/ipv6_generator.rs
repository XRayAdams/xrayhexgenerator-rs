// (c) 2025,26 Konstantin Adamov, licensed under MIT

use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct IPv6Generator;

/// Implementation of BaseGenerator for generating random IPv6 addresses.
impl BaseGenerator for IPv6Generator {
    fn name(&self) -> &str {
        "IPv6 Address"
    }
    fn digist_are_editable(&self) -> bool {
        false
    }
    fn default_digits(&self) -> usize {
        32
    }

    fn generate(&self, lines: usize, _digits: usize, upper_case: bool) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();

        for _ in 0..lines {
            let groups: Vec<String> = (0..8)
                .map(|_| {
                    let word: u16 = rng.gen_range(0u16..=65535u16);
                    if upper_case {
                        format!("{:04X}", word)
                    } else {
                        format!("{:04x}", word)
                    }
                })
                .collect();
            result.push_str(&groups.join(":"));
            result.push('\n');
        }

        result
    }
}
