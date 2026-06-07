// (c) 2025,26 Konstantin Adamov, licensed under MIT

pub mod actions;
pub mod constants;
pub mod i18n;
pub mod init_icon;
pub mod number_editor;

pub use actions::{AboutAction, WindowActionGroup, create_about_action};
pub use constants::{APP_NAME, APP_ID, SPACING_LARGE, SPACING_MEDIUM};
pub use number_editor::NumberEditor;