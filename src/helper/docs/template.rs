use std::collections::HashMap;

use thiserror::Error as ThisError;

const OPENING_BRACE: char = '{';
const CLOSING_BRACE: char = '}';
const ESCAPE_CHAR: char = '\\';
const OPTIONAL_CHAR: char = '?';

/// The template error type.
#[derive(Debug, ThisError, PartialEq)]
pub enum HalpTemplateError {
    /// Error that might occur if there an opening bract and no closing bract for it.
    #[error("Missing closing bract at index: `{0}`")]
    MissingClosingBract(usize),
    /// Error that might occur if there an closing bract and no opening bract for it.
    #[error("Missing opening bract at index: `{0}`")]
    MissingOpeningBract(usize),
    /// Error that might occur if there an opening bracts but there is a placeholder name in
    #[error("Missing placeholder at index: `{0}`")]
    MissingPlaceholder(usize),
    /// Error that might occur if there is no placeholder with the given name.
    #[error("No such placeholder with name: `{0}`")]
    NoSuhPlaceholder(String),
}

/// Type alias for Template Result.
pub type Result<T> = std::result::Result<T, HalpTemplateError>;

/// Parses the template string and replaces the correct values from the values map.
///
/// # Arguments
/// - `template`: The template string to parse.
/// - `values_map`: The values map to replace the values from.
///
/// # Returns
/// This method returns a Result type. On successful, it contains a `String` with the parsed template.
///
/// # Example
/// ```
/// # use halp::helper::docs::template::parse_template;
/// # use std::collections::HashMap;
/// let template = "Hello {name}!";
/// let mut values_map = HashMap::new();
/// values_map.insert("name".to_string(), "Ferris".to_string());
/// let parsed = parse_template(template, &values_map);
/// assert_eq!(parsed, Ok("Hello Ferris!".to_string()));
/// ```
///
/// # Errors
/// - [`HalpTemplateError::MissingClosingBract`]: If there an opening bract and no closing bract for it.
/// - [`HalpTemplateError::MissingOpeningBract`]: If there an closing bract and no opening bract for it.
/// - [`HalpTemplateError::MissingPlaceholder`]: If there an opening bracts but there is a placeholder name in.
/// - [`HalpTemplateError::NoSuhPlaceholder`]: If there is no key in the values map with the given name.
pub fn parse_template(template: &str, values_map: &HashMap<String, String>) -> Result<String> {
    const BUFFER_CAPACITY: usize = 13;
    let mut processed_string = String::with_capacity(template.len() + BUFFER_CAPACITY);
    let mut buffer = String::with_capacity(BUFFER_CAPACITY);
    let mut op_buffer = String::with_capacity(BUFFER_CAPACITY);
    let mut optional = false;
    let mut iter = template.chars();
    let mut nested_level = 0u8;
    loop {
        let Some(c) = iter.next() else { break; };
        match c {
            ESCAPE_CHAR => {
                let Some(next) = iter.next() else {
                    buffer.push(c);
                    continue;
                };
                if next == OPENING_BRACE || next == CLOSING_BRACE || next == OPTIONAL_CHAR {
                    buffer.push(next);
                }
            }
            OPENING_BRACE => {
                nested_level += 1;
                if optional {
                    op_buffer.push_str(&buffer);
                } else {
                    processed_string.push_str(&buffer);
                }
                buffer.clear()
            }
            CLOSING_BRACE => {
                let key = buffer.drain(..).collect::<String>();
                if key.is_empty() {
                    if nested_level == 0 {
                        return Err(HalpTemplateError::MissingOpeningBract(
                            processed_string.len() + buffer.len(),
                        ));
                    }
                    nested_level -= 1;
                    continue;
                }
                if let Some(val) = values_map.get(&key) {
                    processed_string.push_str(&op_buffer);
                    processed_string.push_str(val);
                    optional = false;
                } else if !optional {
                    return Err(HalpTemplateError::NoSuhPlaceholder(key));
                }
                op_buffer.clear();
                nested_level -= 1;
            }
            OPTIONAL_CHAR => {
                optional = true;
            }
            _ => buffer.push(c),
        }
    }
    if !buffer.is_empty() {
        processed_string += &buffer;
    }
    if nested_level != 0 {
        return Err(HalpTemplateError::MissingClosingBract(
            processed_string.len(),
        ));
    }
    Ok(processed_string)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const CHEAT_SH_URL_TEMPLATE: &str = "https://cheat.sh/{cmd}{?/{subcommand}}{? {args}}";

    fn values_map() -> HashMap<String, String> {
        let mut values_map = HashMap::new();
        values_map.insert("cmd".to_string(), "git".to_string());
        values_map.insert("subcommand".to_string(), "commit".to_string());
        values_map.insert("args".to_string(), "-a".to_string());
        values_map
    }

    #[test]
    fn test_parse_cheat_dot_sh_template() -> Result<()> {
        let result = parse_template(CHEAT_SH_URL_TEMPLATE, &values_map())?;
        assert_eq!(result, "https://cheat.sh/git/commit -a");
        Ok(())
    }

    #[test]
    fn test_parse_cheat_dot_sh_template_no_args() -> Result<()> {
        let mut values_map = values_map();
        values_map.remove("args");
        let result = parse_template(CHEAT_SH_URL_TEMPLATE, &values_map)?;
        assert_eq!(result, "https://cheat.sh/git/commit");
        Ok(())
    }

    #[test]
    fn test_dose_nothing() -> Result<()> {
        let result = parse_template("https://cheat.sh/git/commit -a", &values_map())?;
        assert_eq!(result, "https://cheat.sh/git/commit -a");
        Ok(())
    }

    #[test]
    fn test_basic_template_with_one_placeholder() -> Result<()> {
        let result = parse_template("{cmd}", &values_map())?;
        assert_eq!(result, "git");
        Ok(())
    }

    #[test]
    fn test_basic_template_with_one_placeholder_and_text() -> Result<()> {
        let result = parse_template("man {cmd}", &values_map())?;
        assert_eq!(result, "man git");
        Ok(())
    }

    #[test]
    fn test_basic_template_with_more_than_one_placeholder() -> Result<()> {
        let result = parse_template("{cmd} {subcommand}", &values_map())?;
        assert_eq!(result, "git commit");
        Ok(())
    }

    #[test]
    fn test_basic_template_with_more_then_one_placeholder_and_text() -> Result<()> {
        let result = parse_template("info {cmd}/{subcommand}", &values_map())?;
        assert_eq!(result, "info git/commit");
        Ok(())
    }

    #[test]
    fn test_nested_options_lv_one() -> Result<()> {
        let result = parse_template("{cmd}{?/{subcommand}}", &values_map())?;
        assert_eq!(result, "git/commit");
        Ok(())
    }

    #[test]
    fn test_nested_options_lv_two() -> Result<()> {
        let result = parse_template("{cmd}{?/{subcommand}{? {args}}}", &values_map())?;
        assert_eq!(result, "git/commit -a");
        Ok(())
    }

    #[test]
    fn test_nested_options_lv_three() -> Result<()> {
        let result = parse_template("{cmd}{?/{subcommand}{? {args}{? {args2}}}}", &values_map())?;
        assert_eq!(result, "git/commit -a");
        Ok(())
    }

    #[test]
    fn test_nested_options_lv_three_2() -> Result<()> {
        let result = parse_template("{cmd}{?/{subcommand}{?/{args}{?/{args2}}}}", &values_map())?;
        assert_eq!(result, "git/commit/-a");
        Ok(())
    }

    #[test]
    fn test_escape() -> Result<()> {
        let result = parse_template("\\{cmd\\}", &values_map())?;
        assert_eq!(result, "{cmd}");
        Ok(())
    }

    #[test]
    fn test_escape_with_placeholder() -> Result<()> {
        let result = parse_template("\\{cmd\\} {subcommand} \\", &values_map())?;
        assert_eq!(result, "{cmd} commit \\");
        Ok(())
    }

    #[test]
    fn test_escape_with_placeholder_and_option() -> Result<()> {
        let result = parse_template("\\{cmd\\}{?/{subcommand}} {?args}", &values_map())?;
        assert_eq!(result, "{cmd}/commit -a");
        Ok(())
    }

    #[test]
    fn test_none_exist_placeholder_error() {
        let result = parse_template("{cmd}-{subcommand} {argsn}", &values_map());
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("This should fail"),
            HalpTemplateError::NoSuhPlaceholder("argsn".to_string())
        );
    }

    #[test]
    fn test_no_placeholder_error_handle() -> Result<()> {
        let result = parse_template("search: {cmd}-{?subcommand} {?argsn}", &values_map())?;
        assert_eq!(result, "search: git-commit ");
        Ok(())
    }
}
