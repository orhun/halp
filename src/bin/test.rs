use clap::{ColorChoice, Parser};

#[derive(Parser, Debug)]
#[command(bin_name = "test", version, disable_colored_help = true, color = ColorChoice::Never)]
struct Args {}

fn main() {
    Args::parse();
}
