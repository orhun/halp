use clap::CommandFactory;
use clap_mangen::Man;
use halp::cli::CliArgs;
use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

/// Man page can be created with:
///
/// ```sh
/// cargo run --bin halp-mangen
/// ````
///
/// in a directory specified by the environment variable OUT_DIR.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out_path = PathBuf::from(out_dir).join(concat!(env!("CARGO_PKG_NAME"), ".1"));
    let app = CliArgs::command();
    let man = Man::new(app);
    let mut buffer = Vec::<u8>::new();
    man.render(&mut buffer)?;
    fs::write(&out_path, buffer)?;
    println!("Man page is generated at {out_path:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_manpage() -> Result<()> {
        main()
    }
}
