// (c) 2025,26 Konstantin Adamov, licensed under MIT

use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct HexColorWithAlphaGenerator;

/// Implementation of BaseGenerator for generating Hex color codes with alpha channel.
/// The Hex color codes are represented as 8-digit hexadecimal strings.
impl BaseGenerator for HexColorWithAlphaGenerator {
    fn name(&self) -> &str {
        "Hex Color with Alpha"
    }
    fn digist_are_editable(&self) -> bool {
        false
    }
    fn default_digits(&self) -> usize {
        8
    }
    fn generate(&self, lines: usize, _digits: usize, upper_case: bool) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();   
        for _ in 0..lines {
            let mut color_code = String::new();
            for _ in 0..8 {
                let hex_digit = rng.gen_range(0..16);
                if upper_case {
                    color_code.push_str(&format!("{:X}", hex_digit));
                } else {
                    color_code.push_str(&format!("{:x}", hex_digit));
                }
            }
            result.push_str(&format!("#{}", color_code));
            result.push('\n');
        }
        result
    }
}
