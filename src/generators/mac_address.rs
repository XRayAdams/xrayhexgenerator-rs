use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct MacAddressGenerator;

/// Implementation of BaseGenerator for generating MAC addresses.
impl BaseGenerator for MacAddressGenerator {
    fn name(&self) -> &str {
        "MAC Address"
    }
    fn digist_are_editable(&self) -> bool {
        false
    }
    fn default_digits(&self) -> usize {
        12
    }
    
    fn generate(&self, lines: usize, _digits: usize, upper_case: bool) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();

        for _ in 0..lines {
            let mac: Vec<String> = (0..6)
                .map(|_| {
                    let byte = rng.gen_range(0..=255);
                    if upper_case {
                        format!("{:02X}", byte)
                    } else {
                        format!("{:02x}", byte)
                    }
                })
                .collect();
            result.push_str(&mac.join(":"));
            result.push('\n');
        }

        result
    }
}
