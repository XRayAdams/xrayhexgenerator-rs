use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct ByteSequenceGenerator;

/// Implementation of BaseGenerator for generating byte sequences.
/// The byte sequences are represented as hexadecimal strings.
impl BaseGenerator for ByteSequenceGenerator {
    fn name(&self) -> &str {
        "Byte Sequence"
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
        let mut num_hex_chars = digits;
        if num_hex_chars % 2 != 0 {
            num_hex_chars = (num_hex_chars / 2) * 2;
        }
        let num_bytes = num_hex_chars / 2;

        for _ in 0..lines {
            let mut line = String::new();
            for j in 0..num_bytes {
                if upper_case {
                    line.push_str(&format!("{:X}", rng.gen_range(0..16)));
                    line.push_str(&format!("{:X}", rng.gen_range(0..16)));
                } else {
                    line.push_str(&format!("{:x}", rng.gen_range(0..16)));
                    line.push_str(&format!("{:x}", rng.gen_range(0..16)));
                }
                if j < num_bytes - 1 {
                    line.push(' ');
                }
            }
            result.push_str(&line);
            result.push('\n');
        }
        result
    }
}   
