use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Default cheat sheet provider URL.
const CHEAT_SH_URL_TEMPLATE: &str = "https://cheat.sh/{cmd}{?/{subcommand}}{? {args}}";

/// User agent for the cheat sheet provider.
///
/// See <https://github.com/chubin/cheat.sh/blob/83bffa587b6c1048cbcc40ea8fa6af675203fd5f/bin/app.py#L76>
const CHEAT_SH_USER_AGENT: &str = "fetch";

/// EG page provider URL.
const EG_PAGES_URL_TEMPLATE: &str =
    "https://raw.githubusercontent.com/srsudar/eg/master/eg/examples/{cmd}.md";

/// The default cheatsheets provider URL.
const CHEATSHEETS_URL_TEMPLATE: &str =
    "https://raw.githubusercontent.com/cheat/cheatsheets/master/{cmd}";

/// Plz menu config.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlzMenu {
    /// The default selected poison in the menu.
    pub selected_pos: PlzMenuSelection,
    /// The menu entries.
    pub entries: Vec<PlzMenuEntry>,
}

/// Plz menu selection position.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ValueEnum)]
pub enum PlzMenuSelection {
    /// The first item in the menu.
    Start,
    /// The middle item in the menu (in case of even number of items, the first item in the second half).
    Center,
    /// The last item in the menu.
    End,
}

/// Plz menu item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlzMenuEntry {
    /// The string to display in the menu.
    pub display_msg: String,
    /// The operation to perform. and its arguments.
    pub operation: HashMap<String, String>,
}

impl Default for PlzMenu {
    fn default() -> Self {
        PlzMenu {
            selected_pos: PlzMenuSelection::Center,
            entries: vec![
                PlzMenuEntry {
                    display_msg: "Show man page".to_string(),
                    operation: HashMap::from([("run".to_string(), "man {cmd}".to_string())]),
                },
                PlzMenuEntry {
                    display_msg: "Show cheat.sh page".to_string(),
                    operation: HashMap::from([
                        ("fetch".to_string(), CHEAT_SH_URL_TEMPLATE.to_string()),
                        ("user-agent".to_string(), CHEAT_SH_USER_AGENT.to_string()),
                    ]),
                },
                PlzMenuEntry {
                    display_msg: "Show eg page".to_string(),
                    operation: HashMap::from([(
                        "fetch".to_string(),
                        EG_PAGES_URL_TEMPLATE.to_string(),
                    )]),
                },
                PlzMenuEntry {
                    display_msg: "Show cheatsheets page".to_string(),
                    operation: HashMap::from([(
                        "fetch".to_string(),
                        CHEATSHEETS_URL_TEMPLATE.to_string(),
                    )]),
                },
            ],
        }
    }
}
