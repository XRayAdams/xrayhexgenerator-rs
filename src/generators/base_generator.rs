/// Defines the BaseGenerator trait for generating hex data.
pub trait BaseGenerator {
    fn name(&self) -> &str;
    fn generate(&self, lines: usize, digits: usize, upper_case: bool) -> String;
    fn digist_are_editable(&self) -> bool;
    fn default_digits(&self) -> usize;
}
