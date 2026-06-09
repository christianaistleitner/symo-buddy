mod config;

use clap::Parser;
use config::load_config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "config.ini",
        value_name = "filename",
        long_help = "Path to a config file."
    )]
    config_file: String,
}

fn main() {
    // Parse the command line arguments
    let args = Args::parse();

    // Load the configuration from the user-specified path or the default
    let config = load_config(&args.config_file);

    println!("Hello, world!");
}
