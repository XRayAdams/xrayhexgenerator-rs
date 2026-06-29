// (c) 2025,26 Konstantin Adamov, licensed under MIT

use super::base_generator::BaseGenerator;
use rand::Rng;

pub struct ShortIdGenerator;

/// Implementation of BaseGenerator for generating short IDs (NanoID-style).
impl BaseGenerator for ShortIdGenerator {
    fn name(&self) -> &str {
        "Short ID (NanoID)"
    }
    
    fn digist_are_editable(&self) -> bool {
        true
    }
    
    fn default_digits(&self) -> usize {
        21
    }

    fn generate(&self, lines: usize, digits: usize, _upper_case: bool) -> String {
        // URL-safe Base64 alphabet (RFC 4648): A-Z a-z 0-9 - _
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        
        let mut result = String::new();
        let mut rng = rand::thread_rng();

        for _ in 0..lines {
            let mut output = String::new();
            for _ in 0..digits {
                // Use gen_range(0..64) to pick one of 64 chars
                let idx = rng.gen_range(0..ALPHABET.len());
                output.push(ALPHABET[idx] as char);
            }
            result.push_str(&output);
            result.push('\n');
        }
        
        result
    }
}
