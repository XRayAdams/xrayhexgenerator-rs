use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct HexColorGenerator;

/// Implementation of BaseGenerator for generating Hex color codes.
/// The Hex color codes are represented as 6-digit hexadecimal strings.
impl BaseGenerator for HexColorGenerator {
    fn name(&self) -> &str {
        "Hex Color"
    }
    fn digist_are_editable(&self) -> bool {
        false
    }
    fn default_digits(&self) -> usize {
        6
    }
    fn generate(&self, lines: usize, _digits: usize, upper_case: bool) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();   
        for _ in 0..lines {
            let mut color_code = String::new();
            for _ in 0..6 {
                if upper_case {
                    color_code.push_str(&format!("{:X}", rng.gen_range(0..16)));
                } else {
                    color_code.push_str(&format!("{:x}", rng.gen_range(0..16)));
                }
            }
            result.push_str(&format!("#{}", color_code));
            result.push('\n');
        }
        result
    }
}
