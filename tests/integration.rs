use halp::cli::CliArgs;
use halp::error::Result;
use pretty_assertions::assert_eq;

/// Test binary.
const BIN: &str = env!("CARGO_BIN_EXE_halp-test");

#[test]
fn get_argument_help() -> Result<()> {
    let args = CliArgs {
        bin: Some(BIN.to_string()),
        ..Default::default()
    };
    let mut output = Vec::new();
    halp::run(args, &mut output)?;
    println!("{}", String::from_utf8_lossy(&output));
    assert_eq!(
        r#"(°ロ°)  checking 'test -v'
(×﹏×)      fail '-v' argument not found.
(°ロ°)  checking 'test -V'
\(^ヮ^)/ success '-V' argument found!
halp 0.1.0
(°ロ°)  checking 'test -h'
\(^ヮ^)/ success '-h' argument found!
Usage: test

Options:
  -h, --help     Print help
  -V, --version  Print version"#,
        String::from_utf8_lossy(&output)
            .replace('\r', "")
            .replace(BIN, "test")
            .replace("0.1.0", env!("CARGO_PKG_VERSION"))
            .trim()
    );
    Ok(())
}
