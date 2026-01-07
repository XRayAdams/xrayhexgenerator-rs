use libadwaita as adw;
use relm4::prelude::*;

mod generators;
mod helpers;
mod app;
use helpers::constants::{APP_ID};
use helpers::init_icon::init_app_icon;
use app::App;


fn main() {
    adw::init().expect("Failed to initialize Libadwaita");

    init_app_icon();

    let app = RelmApp::new(APP_ID);
    app.run::<App>(());
}
