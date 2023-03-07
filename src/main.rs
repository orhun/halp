use clap::Parser;
use halp::args::Args;
use std::io;
use std::process;

fn main() {
    let args = Args::parse();
    let mut stdout = io::stdout();
    match halp::run(args, &mut stdout) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
