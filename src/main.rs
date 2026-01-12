use libadwaita as adw;
use relm4::prelude::*;
use libadwaita::prelude::ApplicationExt;

mod generators;
mod helpers;
mod app;
use helpers::constants::{APP_ID};
use helpers::init_icon::init_app_icon;
use app::App;


fn main() {
    gtk4::init().expect("Failed to initialize GTK");
    
    let gtk_app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    gtk_app.connect_activate(|_| {
        init_app_icon();    
    });
    
    let app = RelmApp::from_app(gtk_app);
    app.run::<App>(());

}
