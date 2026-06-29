// (c) 2025,26 Konstantin Adamov, licensed under MIT

use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct Base32Generator;

/// Implementation of BaseGenerator for generating random Base32 strings.
impl BaseGenerator for Base32Generator {
    fn name(&self) -> &str {
        "Base32 String"
    }
    
    fn digist_are_editable(&self) -> bool {
        true
    }
    
    fn default_digits(&self) -> usize {
        16
    }

    fn generate(&self, lines: usize, digits: usize, _upper_case: bool) -> String {
        // Base32 RFC 4648 alphabet: A-Z and 2-7 (32 chars)
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        
        let mut result = String::new();
        let mut rng = rand::thread_rng();

        for _ in 0..lines {
            let mut output = String::new();
            for _ in 0..digits {
                // Randomly select a char from the 32-char alphabet
                let idx = rng.gen_range(0..ALPHABET.len());
                output.push(ALPHABET[idx] as char);
            }
            result.push_str(&output);
            result.push('\n');
        }
        
        result
    }
}
