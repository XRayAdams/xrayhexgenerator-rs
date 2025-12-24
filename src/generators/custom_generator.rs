use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct CustomGenerator;

/// Implementation of BaseGenerator for generating custom hex strings.
impl BaseGenerator for CustomGenerator {
    fn name(&self) -> &str {
        "Custom Hex"
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
            let hex_string: String = (0..digits)
                .map(|_| {
                    let n = rng.gen_range(0..16);
                    if upper_case {
                        format!("{:X}", n)
                    } else {
                        format!("{:x}", n)
                    }
                })
                .collect();
            result.push_str(&hex_string);
            result.push('\n');
        }

        result
    }
}
