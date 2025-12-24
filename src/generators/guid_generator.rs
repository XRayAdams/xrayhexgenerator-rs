use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct GUIDGenerator;

/// Implementation of BaseGenerator for generating GUIDs.
/// The GUIDs are represented in the standard 8-4-4-4-12 hexadecimal format.
impl BaseGenerator for GUIDGenerator {
    fn name(&self) -> &str {
        "GUID"
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
            let mut guid = String::new();
            for (i, &len) in [8, 4, 4, 4, 12].iter().enumerate() {
                for _ in 0..len {
                    let hex = if upper_case {
                        format!("{:X}", rng.gen_range(0..16))
                    } else {
                        format!("{:x}", rng.gen_range(0..16))
                    };
                    guid.push_str(&hex);
                }
                if i < 4 {
                    guid.push('-');
                }
            }
            result.push_str(&guid);
            result.push('\n');
        }
        result
    }
}
