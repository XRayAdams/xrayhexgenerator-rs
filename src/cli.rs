// (c) 2025,26 Konstantin Adamov, licensed under MIT

use clap::{CommandFactory, FromArgMatches, Parser};
use crate::tr;

#[derive(Parser, Debug)]
#[command(name = "xrayhexgenerator")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generate various types of hexadecimal values", long_about = None)]
#[command(disable_version_flag = true)]
#[command(arg_required_else_help = false)]
pub struct Cli {
    /// Generator type: custom, mac, eui64, ipv4, ipv6, guid, hexcolor, hexalpha, byteseq, prefixed
    #[arg(short = 'g', long = "generator", value_name = "TYPE")]
    pub generator: Option<String>,

    /// Number of lines to generate
    #[arg(short = 'l', long = "lines", value_name = "X", default_value = "1")]
    pub lines: usize,

    /// Number of digits (where applicable)
    #[arg(short = 'd', long = "digits", value_name = "Y")]
    pub digits: Option<usize>,

    /// Use uppercase hexadecimal characters
    #[arg(short = 'u', long = "uppercase")]
    pub uppercase: bool,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    _version: Option<bool>,
}

impl Cli {
    pub fn parse_args() -> Self {
        let cmd = Cli::command()
            .about(tr!("Generate various types of hexadecimal values"))
            .disable_help_flag(true)
            .arg(
                clap::Arg::new("help")
                    .short('h')
                    .long("help")
                    .action(clap::ArgAction::Help)
                    .help(tr!("Print help"))
            )
            .mut_arg("generator", |a| {
                a.help(tr!("Generator type: custom, mac, eui64, ipv4, ipv6, guid, hexcolor, hexalpha, byteseq, prefixed"))
            })
            .mut_arg("lines", |a| {
                a.help(tr!("Number of lines to generate"))
            })
            .mut_arg("digits", |a| {
                a.help(tr!("Number of digits (where applicable)"))
            })
            .mut_arg("uppercase", |a| {
                a.help(tr!("Use uppercase hexadecimal characters"))
            })
            .mut_arg("_version", |a| {
                a.help(tr!("Print version"))
            });

        let matches = cmd.get_matches();
        Cli::from_arg_matches(&matches).expect("Failed to parse CLI arguments")
    }

    pub fn is_cli_mode() -> bool {
        std::env::args().len() > 1
    }
}
