use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

use colored::Colorize;
use console::{style, Style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use crate::config::{plz_menu::PlzMenuSelection, Config};
use crate::error::{Error, Result};
use crate::helper::args::FAIL_EMOTICON;
use crate::helper::docs::cmd_parse::parse_cmd;
use crate::helper::docs::handlers::{command::CommandHandler, fetch::FetchHandler, Handler};
use crate::helper::docs::template::parse_template;

/// Command parser.
pub mod cmd_parse;
/// Plz menu operation handlers.
pub mod handlers;
/// Plz menu template parser.
pub mod template;

/// Shows documentation/usage help about the given command.
pub fn get_docs_help<Output: Write>(cmd: &str, config: Config, output: &mut Output) -> Result<()> {
    let mut menu_options = config
        .plz_menu
        .entries
        .iter()
        .map(|e| e.display_msg.as_str())
        .collect::<Vec<&str>>();
    use PlzMenuSelection as Selection;
    let mut selection = match config.plz_menu.selected_pos {
        Selection::Start => Some(0),
        Selection::Center => Some(menu_options.len() / 2),
        Selection::End => Some(menu_options.len() - 1),
    };
    menu_options.push("Exit");
    let values_map = build_the_values_map(cmd);
    loop {
        selection = Select::with_theme(&get_selection_theme())
            .with_prompt("Select operation")
            .default(selection.unwrap_or_default())
            .items(&menu_options)
            .interact_on_opt(&Term::stderr())?;
        // Exit conditions
        let Some(selection) = selection else { break Ok(()); };
        let Some(entry) = config.plz_menu.entries.get(selection) else { break Ok(()); };
        let operation_iter = entry.operation.iter();
        // if there is no key, then return an error, the operation key is required.
        let (mut op_key, mut op_value) = (None, None);
        // Create a new map with the capacity of the values map minus the first entry, to contain the parsed values.
        let mut parsed_map = HashMap::with_capacity(values_map.len() - 1);
        for (key, value) in operation_iter {
            // If the operation key is not set, then check if the key is a valid operation key.
            if op_key.is_none()
                && (key == "fetch"
                    || key == "url"
                    || key == "command"
                    || key == "run"
                    || key == "file")
            {
                op_key = Some(key.clone());
                op_value = Some(value.clone());
                continue;
            }
            parsed_map.insert(key.as_str(), parse_template(value, &values_map)?);
        }
        // If the operation key is not set, then return an error, the operation key is required.
        let (Some(op_key), Some(op_value)) = (op_key, op_value) else {return Err(Error::PlzMenuNoOperation)};
        let op_value = parse_template(&op_value, &values_map)?;
        let result = match op_key.as_str() {
            "fetch" | "url" => FetchHandler.handle(op_value, &entry.operation),
            "command" | "run" => CommandHandler.handle(op_value, &entry.operation),
            "file" => FetchHandler.handle(op_value, &entry.operation),
            _ => break Err(Error::PlzMenuInvalidOperation(op_key.to_string())),
        };
        let Ok(result) = result else {
            writeln!(output, "{} {}", FAIL_EMOTICON.magenta(),
                     result.expect_err("error").to_string().red().bold())?;
            continue;
        };
        let Some(page) = result else { continue; };
        // Show the page using the user selected pager or write it directly into the output
        if let Some(pager) = config.pager_command.as_ref() {
            let mut process = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", pager])
                    .stdin(Stdio::piped())
                    .spawn()
            } else {
                Command::new("sh")
                    .args(["-c", pager])
                    .stdin(Stdio::piped())
                    .spawn()
            }?;
            if let Some(stdin) = process.stdin.as_mut() {
                writeln!(stdin, "{}", page)?;
                process.wait()?;
            }
        } else {
            writeln!(output, "{}", page)?;
        }
    }
}

/// Builds the values map for the template engine.
#[inline(always)]
fn build_the_values_map(cmd: &str) -> HashMap<String, String> {
    let mut map = HashMap::with_capacity(5);
    parse_cmd(cmd, &mut map); // This will add at least one entry and at most 3 entries
    if map.capacity() < 3 {
        map.shrink_to(map.capacity() + 2);
    }
    map.insert(
        "halp-version".to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
    );
    map.insert("os".to_string(), std::env::consts::OS.to_string());
    map
}

/// Returns the theme for selection prompt.
fn get_selection_theme() -> ColorfulTheme {
    ColorfulTheme {
        defaults_style: Style::new().for_stderr().cyan(),
        prompt_style: Style::new().for_stderr().bold(),
        prompt_prefix: style("(ﾉ´ヮ`)ﾉ*: ･ﾟ\n".to_string()).for_stderr().magenta(),
        prompt_suffix: style("›".to_string()).for_stderr().magenta().bright(),
        success_prefix: style("❤".to_string()).for_stderr().magenta(),
        success_suffix: style("·".to_string()).for_stderr().black().bright(),
        error_prefix: style("✘".to_string()).for_stderr().red(),
        error_style: Style::new().for_stderr().red(),
        hint_style: Style::new().for_stderr().black().bright(),
        values_style: Style::new().for_stderr().green(),
        active_item_style: Style::new().for_stderr().green().bold(),
        inactive_item_style: Style::new().for_stderr().italic(),
        active_item_prefix: style("✧".to_string()).for_stderr().magenta().bold(),
        inactive_item_prefix: style(" ".to_string()).for_stderr(),
        checked_item_prefix: style("❤".to_string()).for_stderr().magenta(),
        unchecked_item_prefix: style("❤".to_string()).for_stderr().black(),
        picked_item_prefix: style("❯".to_string()).for_stderr().green(),
        unpicked_item_prefix: style(" ".to_string()).for_stderr(),
        inline_selections: true,
    }
}
