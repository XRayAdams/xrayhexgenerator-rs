use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct PrefixedHexGenerator;
/// Implementation of BaseGenerator for generating prefixed hex strings.
/// 
/// The generated hex strings start with "0x" followed by the specified number of hexadecimal digits.
impl BaseGenerator for PrefixedHexGenerator {
    fn name(&self) -> &str {
        "Prefixed Hex"
    }
    fn digist_are_editable(&self) -> bool {
        true
    }
    fn default_digits(&self) -> usize {
        16
    }
    fn generate(&self, lines: usize, digits: usize, upper_case: bool) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();       
        for _ in 0..lines {
            let mut hex_string = String::from("0x");
            for _ in 0..digits {
                let n = rng.gen_range(0..16);
                if upper_case {
                    hex_string.push_str(&format!("{:X}", n));
                } else {
                    hex_string.push_str(&format!("{:x}", n));
                }
            }
            result.push_str(&hex_string);
            result.push('\n');
        }
        result
    }
}
