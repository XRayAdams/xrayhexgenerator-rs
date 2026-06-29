// (c) 2025,26 Konstantin Adamov, licensed under MIT

use libadwaita as adw;
use relm4::prelude::*;
use libadwaita::prelude::ApplicationExt;

mod generators;
mod helpers;
mod app;
mod cli;
use helpers::constants::{APP_ID, GENERATORS_LIST};
use helpers::i18n::init_i18n;
use helpers::init_icon::init_app_icon;
use app::App;
use cli::Cli;
use generators::base_generator::BaseGenerator;
use generators::custom_generator::CustomGenerator;
use generators::mac_address::MacAddressGenerator;
use generators::eui64_generator::Eui64Generator;
use generators::ipv4_generator::IPv4Generator;
use generators::ipv6_generator::IPv6Generator;
use generators::guid_generator::GUIDGenerator;
use generators::hex_color::HexColorGenerator;
use generators::hex_color_with_alpha::HexColorWithAlphaGenerator;
use generators::byte_sequence::ByteSequenceGenerator;
use generators::prefixed_hex::PrefixedHexGenerator;
use generators::base32_generator::Base32Generator;
use generators::shortid_generator::ShortIdGenerator;

fn run_cli_mode() {
    let args = Cli::parse_args();
    
    // Determine which generator to use
    let generator: Box<dyn BaseGenerator> = match args.generator.as_deref() {
        Some("custom") => Box::new(CustomGenerator),
        Some("mac") => Box::new(MacAddressGenerator),
        Some("eui64") => Box::new(Eui64Generator),
        Some("ipv4") => Box::new(IPv4Generator),
        Some("ipv6") => Box::new(IPv6Generator),
        Some("guid") => Box::new(GUIDGenerator),
        Some("hexcolor") => Box::new(HexColorGenerator),
        Some("hexalpha") => Box::new(HexColorWithAlphaGenerator),
        Some("byteseq") => Box::new(ByteSequenceGenerator),
        Some("prefixed") => Box::new(PrefixedHexGenerator),
        Some("base32") => Box::new(Base32Generator),
        Some("shortid") => Box::new(ShortIdGenerator),
        Some(unknown) => {
            eprintln!("{}", tr!("Unknown generator: {}. Using 'custom' instead.").replace("{}", unknown));
            eprintln!("{}", tr!("Available generators: {}").replace("{}", GENERATORS_LIST));
            Box::new(CustomGenerator)
        }
        None => {
            eprintln!("{}", tr!("No generator specified. Use -g <type> to select a generator."));
            eprintln!("{}", tr!("Available generators: {}").replace("{}", GENERATORS_LIST));
            std::process::exit(1);
        }
    };
    
    // Get the number of digits, or use the default for the generator
    let digits = args.digits.unwrap_or_else(|| generator.default_digits());
    
    // Generate the output
    let output = generator.generate(args.lines, digits, args.uppercase);
    
    // Print the result
    print!("{}", output);
}

fn main() {
    init_i18n();

    // Check if we're running in CLI mode
    if Cli::is_cli_mode() {
        run_cli_mode();
        return;
    }
    
    // Otherwise, run the GUI
    gtk4::init().expect(&tr!("Failed to initialize GTK"));
    
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

