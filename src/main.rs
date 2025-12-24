use gtk4::{prelude::*};
use libadwaita as adw;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, TextView, 
           MenuButton, gio, HeaderBar, IconTheme, ScrolledWindow, PolicyType, ComboBoxText, Switch, Label, Align, glib};
use std::cell::RefCell;
use std::rc::Rc;

mod generators;
mod view_model;
mod helpers;
use view_model::AppViewModel;
use helpers::utils::create_labeled_spin;

const SPACING_MEDIUM: i32 = 12;
const SPACING_LARGE: i32 = 18;

fn main() {
    adw::init().expect("Failed to initialize Libadwaita");
    let app = Application::builder()
        .application_id("app.rayadams.xrayhexgenerator")
        .flags(gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(build_ui);

    app.run();

}

fn build_ui(app: &Application) {
    let display = gtk4::gdk::Display::default().expect("Could not get default display.");
    let icon_theme = IconTheme::for_display(&display);

        // Check if running in Snap environment
    if let Ok(snap_path) = std::env::var("SNAP") {
        let assets_path = std::path::Path::new(&snap_path).join("assets");
        icon_theme.add_search_path(assets_path);
    } else {
        // Fallback for local development
        icon_theme.add_search_path("assets");

        // Check paths relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // 1. Assets next to executable (e.g. portable tarball)
                let local_assets = exe_dir.join("assets");
                if local_assets.exists() {
                    icon_theme.add_search_path(local_assets);
                }

                // 2. Standard Linux install: ../share/loremgenerator/assets
                // (assuming binary is in /usr/bin or /usr/local/bin)
                if let Some(prefix) = exe_dir.parent() {
                    let system_assets = prefix.join("share").join("xrayhexgenerator").join("assets");
                    if system_assets.exists() {
                        icon_theme.add_search_path(system_assets);
                    }
                }
            }
        }
    }

    let view_model = Rc::new(RefCell::new(AppViewModel::new()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hex Generator")
        .default_width(900)
        .default_height(600)
        .build();

    let header_bar = HeaderBar::new();

    let menu = gio::Menu::new();
    menu.append(Some("About"), Some("app.about"));
    
    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu)
        .build();
    
    header_bar.pack_end(&menu_button);
    
    window.set_titlebar(Some(&header_bar));

    let about_action = gio::SimpleAction::new("about", None);
    let window_clone = window.clone();

    about_action.connect_activate(move |_, _| {
        let about = adw::AboutWindow::builder()
            .application_name("HEX Generator")
            .application_icon("app.rayadams.xrayhexgenerator")
            .version(AppViewModel::get_app_version())
            .developers(vec!["Konstantin Adamov".to_string()])
            .website("https://github.com/xrayadams/xrayhexgenerator-sr")
            .issue_url("https://github.com/xrayadams/xrayhexgenerator-rs/issues")
            .license_type(gtk4::License::MitX11)
            .transient_for(&window_clone)
            .modal(true)
            .build();
        about.present();
    });
    app.add_action(&about_action);

    let vbox = GtkBox::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_top(SPACING_LARGE)
        .margin_bottom(SPACING_LARGE)
        .margin_start(SPACING_LARGE)
        .margin_end(SPACING_LARGE)
        .spacing(10)
        .build();

    let settings_box = GtkBox::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    // Generator selection
    let generators_box = GtkBox::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(6)
        .halign(Align::Start)
        .build();

    let gen_label = Label::with_mnemonic("_Generator : ");
    let generators_cb = ComboBoxText::new();
    for generator in &view_model.borrow().generators {
        generators_cb.append_text(generator.name());
    }

    // Get saved index 
    let saved_index = view_model.borrow().selected_index;
    let clamped_index = saved_index.min(view_model.borrow().generators.len().saturating_sub(1));
    view_model.borrow_mut().set_selected_index(clamped_index);
    generators_cb.set_active(Some(clamped_index as u32));

    generators_box.append(&gen_label);
    generators_box.append(&generators_cb);
    settings_box.append(&generators_box);
    vbox.append(&settings_box);
    
    // Digits and Lines settings
    let editors_box = GtkBox::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(6)
        .halign(Align::Start)
        .build();
    
    let (box_digits, spin_digits) = create_labeled_spin("Digits", 1, 128, view_model.borrow().digits as i32);
    let (box_lines, spin_lines) = create_labeled_spin("Lines", 1, 100000, view_model.borrow().lines as i32);
    
    spin_digits.set_sensitive(view_model.borrow().is_digits_editable());

    let vm_for_combo = view_model.clone();
    let spin_digits_for_combo = spin_digits.clone();
    generators_cb.connect_changed(
        move |combo| {
            if let Some(index) = combo.active() {
                vm_for_combo.borrow_mut().set_selected_index(index as usize);
                
                let (is_editable, default_digits) = {
                    let vm = vm_for_combo.borrow();
                    let generator = &vm.generators[index as usize];
                    (generator.digist_are_editable(), generator.default_digits())
                };

                // Update digits spin sensitivity and value
                spin_digits_for_combo.set_sensitive(is_editable);
                if !is_editable {
                    spin_digits_for_combo.set_value(default_digits as f64);
                }
            }
        }
    );

    editors_box.append(&box_digits);
    editors_box.append(&box_lines);

    let switch_box = GtkBox::new(gtk4::Orientation::Vertical, 6);
    let switch_label = Label::builder()
        .label("Output in Uppercase")
        .halign(Align::Start)
        .build();
    let switch = Switch::builder()
        .active(view_model.borrow().output_uppercase)
        .halign(Align::Center)
        .build();
    
    switch_box.append(&switch_label);
    switch_box.append(&switch);
    editors_box.append(&switch_box);

    vbox.append(&editors_box);
    //
    let vm = view_model.clone();
    switch.connect_state_set(move |_, state| {
        vm.borrow_mut().set_output_uppercase(state);
        glib::Propagation::Proceed
    });

    let vm = view_model.clone();
    spin_digits.connect_value_changed(move |spin| {
        vm.borrow_mut().set_digits(spin.value() as usize);
    });

    let vm = view_model.clone();
    spin_lines.connect_value_changed(move |spin| {
        vm.borrow_mut().set_lines(spin.value() as usize);
    });

    // output text view
    let output_text_view = TextView::builder()
        .monospace(true)
        .editable(false)
        .tooltip_text("Output")
        .wrap_mode(gtk4::WrapMode::Word)
        .left_margin(SPACING_MEDIUM)
        .right_margin(SPACING_MEDIUM)
        .top_margin(SPACING_MEDIUM)
        .bottom_margin(SPACING_MEDIUM)
        .css_classes(vec!["card"])
        .build();

    let output_scrolled = ScrolledWindow::builder()
        .child(&output_text_view)
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Automatic)
        .vexpand(true)
        .hexpand(true)
        .build();
    vbox.append(&output_scrolled);

    let button_box = GtkBox::new(gtk4::Orientation::Horizontal, SPACING_LARGE);

    let generate_button = gtk4::Button::with_mnemonic("_Generate");
    generate_button.set_halign(gtk4::Align::Start);
    button_box.append(&generate_button);

    let copy_button = gtk4::Button::with_mnemonic("_Copy to Clipboard");
    copy_button.set_halign(gtk4::Align::Start);
    
    // Initially disable copy button if there's no text
    let buffer = output_text_view.buffer();
    copy_button.set_sensitive(!buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).is_empty());

    // Update copy button sensitivity based on text buffer changes
    let copy_button_clone = copy_button.clone();
    buffer.connect_changed(move |buf| {
        let is_empty = buf.text(&buf.start_iter(), &buf.end_iter(), true).is_empty();
        copy_button_clone.set_sensitive(!is_empty);
    });

    let toast_overlay = adw::ToastOverlay::new();
    toast_overlay.set_child(Some(&vbox));
    
    button_box.append(&copy_button);

    vbox.append(&button_box);

    let text_view_clone = output_text_view.clone();
    let vm = view_model.clone();
    generate_button.connect_clicked(move |_| {
        let output_text = vm.borrow().generate_output();
        text_view_clone.buffer().set_text(&output_text);
    });


    let text_view_clone = output_text_view.clone();
    let toast_overlay_clone = toast_overlay.clone();
    copy_button.connect_clicked(move |_| {
        let buffer = text_view_clone.buffer();
        let (start, end) = buffer.bounds();
        let text = buffer.text(&start, &end, true);
        text_view_clone.clipboard().set_text(&text);
        
        let toast = adw::Toast::new("Copied to clipboard");
        toast_overlay_clone.add_toast(toast);
    });

    window.set_titlebar(Some(&header_bar));
    window.set_child(Some(&toast_overlay));
    window.show();
}

