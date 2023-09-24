use std::collections::HashMap;

/// Parses a command string into a HashMap that contains the command parts.
///
/// the command string is expected to be in the format:
/// `<cmd> [<subcommand>] [<args>]`
/// the subcommand and args are optional.
///
/// This function will add at least the `cmd` key to the `values_map` and 3 keys at most.
/// - `cmd`: The command name.
/// - `subcommand`: The command subcommand.
/// - `args`: The command arguments.
///
/// # Example
/// ```
/// # use halp::helper::docs::cmd_parse::parse_cmd;
/// # use std::collections::HashMap;
///
/// let git_commit = "git commit -a";
/// let mut values_map = HashMap::with_capacity(3);
/// parse_cmd(git_commit, &mut values_map);
/// assert!(values_map.contains_key("cmd"));
/// assert_eq!(values_map.get("cmd").unwrap(), "git");
/// assert!(values_map.contains_key("subcommand"));
/// assert_eq!(values_map.get("subcommand").unwrap(), "commit");
/// assert!(values_map.contains_key("args"));
/// assert_eq!(values_map.get("args").unwrap(), "-a");
/// ```
///
/// # Panics
/// THIS FUNCTION WILL PANIC IF THE COMMAND STRING IS EMPTY.
pub fn parse_cmd(cmd: &str, values_map: &mut HashMap<String, String>) {
    let cmd = cmd.trim().to_string();
    let mut iter = cmd.split_whitespace();
    // The `cmd` should be the first value.
    values_map.insert(
        "cmd".to_string(),
        iter.next().expect("The command should exist").to_string(),
    );
    // Parse the rest of the command parts.
    for part in iter {
        if !part.starts_with('-') && !values_map.contains_key("subcommand") {
            values_map.insert("subcommand".to_string(), part.to_string());
        } else if !values_map.contains_key("args") {
            values_map.insert("args".to_string(), part.to_string());
        } else {
            // The args already exists, so we append the part to it.
            let args = values_map.get_mut("args").expect("Unreachable");
            args.push_str(&format!(" {}", part));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_complete_cmd() {
        let git_commit = "git commit -a";
        let mut values_map = HashMap::with_capacity(3);
        parse_cmd(git_commit, &mut values_map);
        assert!(values_map.contains_key("cmd"));
        assert_eq!(values_map.get("cmd"), Some(&"git".to_string()));
        assert!(values_map.contains_key("subcommand"));
        assert_eq!(values_map.get("subcommand"), Some(&"commit".to_string()));
        assert!(values_map.contains_key("args"));
        assert_eq!(values_map.get("args"), Some(&"-a".to_string()));
    }

    #[test]
    fn test_parse_cmd_with_no_args() {
        let git_commit = "git commit";
        let mut values_map = HashMap::with_capacity(3);
        parse_cmd(git_commit, &mut values_map);
        assert!(values_map.contains_key("cmd"));
        assert_eq!(values_map.get("cmd"), Some(&"git".to_string()));
        assert!(values_map.contains_key("subcommand"));
        assert_eq!(values_map.get("subcommand"), Some(&"commit".to_string()));
        assert!(!values_map.contains_key("args"));
    }

    #[test]
    fn test_parse_cmd_with_no_subcommand() {
        let git_commit = "git";
        let mut values_map = HashMap::with_capacity(3);
        parse_cmd(git_commit, &mut values_map);
        assert!(values_map.contains_key("cmd"));
        assert_eq!(values_map.get("cmd"), Some(&"git".to_string()));
        assert!(!values_map.contains_key("subcommand"));
        assert!(!values_map.contains_key("args"));
    }

    #[test]
    fn test_parse_cmd_with_no_subcommand_and_args() {
        let git_commit = "git";
        let mut values_map = HashMap::with_capacity(3);
        parse_cmd(git_commit, &mut values_map);
        assert!(values_map.contains_key("cmd"));
        assert_eq!(values_map.get("cmd"), Some(&"git".to_string()));
        assert!(!values_map.contains_key("subcommand"));
        assert!(!values_map.contains_key("args"));
    }

    #[test]
    fn test_parse_cmd_with_args_and_no_subcommand() {
        let command = "ps -aux";
        let mut values_map = HashMap::with_capacity(3);
        parse_cmd(command, &mut values_map);
        assert!(values_map.contains_key("cmd"));
        assert_eq!(values_map.get("cmd"), Some(&"ps".to_string()));
        assert!(!values_map.contains_key("subcommand"));
        assert!(values_map.contains_key("args"));
        assert_eq!(values_map.get("args"), Some(&"-aux".to_string()));
    }

    #[test]
    fn test_parse_cmd_with_two_args_and_no_subcommand() {
        let command = "ps -aux -l";
        let mut values_map = HashMap::with_capacity(3);
        parse_cmd(command, &mut values_map);
        assert!(values_map.contains_key("cmd"));
        assert_eq!(values_map.get("cmd"), Some(&"ps".to_string()));
        assert!(!values_map.contains_key("subcommand"));
        assert!(values_map.contains_key("args"));
        assert_eq!(values_map.get("args"), Some(&"-aux -l".to_string()));
    }

    #[test]
    fn test_parse_cmd_with_three_args_and_subcommand() {
        let command = "git commit -a -m \"commit message\"";
        let mut values_map = HashMap::with_capacity(3);
        parse_cmd(command, &mut values_map);
        assert!(values_map.contains_key("cmd"));
        assert_eq!(values_map.get("cmd"), Some(&"git".to_string()));
        assert!(values_map.contains_key("subcommand"));
        assert_eq!(values_map.get("subcommand"), Some(&"commit".to_string()));
        assert!(values_map.contains_key("args"));
        assert_eq!(
            values_map.get("args"),
            Some(&"-a -m \"commit message\"".to_string())
        );
    }
}
