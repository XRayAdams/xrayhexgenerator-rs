use crate::generators::base_generator::BaseGenerator;
use crate::generators::mac_address::MacAddressGenerator;
use crate::generators::custom_generator::CustomGenerator;
use crate::generators::byte_sequence::ByteSequenceGenerator;
use crate::generators::guid_generator::GUIDGenerator;
use crate::generators::hex_color::HexColorGenerator;
use crate::generators::hex_color_with_alpha::HexColorWithAlphaGenerator;
use crate::generators::prefixed_hex::PrefixedHexGenerator;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct AppViewModel {
    pub selected_index: usize,
    pub digits: usize,
    pub lines: usize,
    pub output_uppercase: bool,
    #[serde(skip)]
    pub generators: Vec<Box<dyn BaseGenerator>>,
}

impl AppViewModel {
    pub fn new() -> Self {
        Self::load_or_default()
    }

    fn load() -> Option<Self> {
        let path = Self::get_config_path();
        if let Ok(content) = fs::read_to_string(path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    pub fn load_or_default() -> Self {
        if let Some(mut vm) = Self::load() {
            // Reinitialize generators after loading
            vm.generators = Self::get_def_generators();
            vm
        } else {
            Self {
                selected_index: 0,
                digits: 16,
                output_uppercase: true,
                lines: 10,
                generators: Self::get_def_generators(),
            }
        }
    }

    pub fn save(&self) {
        let path = Self::get_config_path();
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, content);
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = gtk4::glib::user_config_dir();
        path.push("xrayhexgenerator");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    fn get_def_generators() -> Vec<Box<dyn BaseGenerator>> {
        vec![
            Box::new(CustomGenerator {}),
            Box::new(MacAddressGenerator {}),
            Box::new(ByteSequenceGenerator {}),
            Box::new(GUIDGenerator {}),
            Box::new(HexColorGenerator {}),
            Box::new(HexColorWithAlphaGenerator {}),
            Box::new(PrefixedHexGenerator {}),
        ]
    }

    pub fn get_app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn set_output_uppercase(&mut self, value: bool) {
        self.output_uppercase = value;
        self.save();
    }

    pub fn set_digits(&mut self, _value: usize) {
        self.digits = _value;
        self.save();
    }

    pub fn set_lines(&mut self, _value: usize) {
        self.lines = _value;
        self.save();
    }

    pub fn set_selected_index(&mut self, index: usize) {
        if self.generators.len() <= index || index >= self.generators.len() {
            return;
        }

        self.selected_index = index;
        let generator = &self.generators[index];
        if !generator.digist_are_editable() {
            self.digits = generator.default_digits() as usize;
        }
        self.save();
    }

    pub fn generate_output(&self) -> String {
        let generator = &self.generators[self.selected_index];

        generator.generate(self.lines, self.digits as usize, self.output_uppercase)
    }

    pub fn is_digits_editable(&self) -> bool {
        let generator = &self.generators[self.selected_index];
        generator.digist_are_editable()
    }
}
