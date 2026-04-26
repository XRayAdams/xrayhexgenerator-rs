// (c) 2025,26 Konstantin Adamov, licensed under MIT

use gettextrs::{LocaleCategory, bindtextdomain, setlocale, textdomain};
use std::path::PathBuf;

pub const GETTEXT_PACKAGE: &str = "xrayhexgenerator";

/// Macro for marking translatable strings
#[macro_export]
macro_rules! tr {
    ($text:expr) => {
        gettextrs::gettext($text)
    };
}

pub fn init_i18n() {
    setlocale(LocaleCategory::LcAll, "");

    let localedir = get_locale_dir();
    bindtextdomain(GETTEXT_PACKAGE, localedir.to_str().unwrap_or("/usr/share/locale"))
        .expect("Failed to bind text domain");
    textdomain(GETTEXT_PACKAGE).expect("Failed to set text domain");
}

fn get_locale_dir() -> PathBuf {
    // 1. Check LOCALEDIR environment variable
    if let Ok(dir) = std::env::var("LOCALEDIR") {
        let path = PathBuf::from(&dir);
        if path.exists() {
            return path;
        }
    }

    // 2. Check relative to executable (for development builds)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            // Check locale/ next to binary
            let build_locale = exe_dir.join("locale");
            if build_locale.exists() {
                return build_locale;
            }

            // Check ../share/locale (typical install layout)
            let share_locale = exe_dir.join("../share/locale");
            if share_locale.exists() {
                return share_locale;
            }
        }
    }

    // 3. Fallback to system locale directory
    PathBuf::from("/usr/share/locale")
}
