use std::fmt::Display;

/// Flag for the help information.
#[derive(Debug)]
pub enum HelpArg {
    /// Default help.
    Help,
    /// Long help.
    LongHelp,
    /// Capital help.
    CapitalHelp,
}

impl HelpArg {
    /// Returns the available variants.
    pub fn variants() -> &'static [Self] {
        &[Self::Help, Self::LongHelp, Self::CapitalHelp]
    }
}

impl Display for HelpArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HelpArg::Help => "-h",
                HelpArg::LongHelp => "--help",
                HelpArg::CapitalHelp => "-H",
            }
        )
    }
}
