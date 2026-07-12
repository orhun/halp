/// Display trait for an argument.
pub trait ArgDisplay {
    /// Returns the string representation.
    fn as_str(&self) -> &'static str;
}

macro_rules! generate_argument {
    ($name: ident,
     $($variant: ident => $str_repr: expr,)+
    ) => {
        /// Argument.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $(
                /// Variant of the argument.
                $variant
            ),+
        }

        impl $name {
            /// Returns the variants.
            pub fn variants() -> Vec<Box<dyn ArgDisplay>> {
                vec![$(Box::new(Self::$variant),)+]
            }

        }

        impl ArgDisplay for $name {
            fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $str_repr,)+
                }
            }
        }
    };
}

generate_argument!(
    VersionArg,
    // Prefer long forms first: some tools (e.g. `ps`) use `-v`/`-h` for other meaning.
    LongVersion => "--version",
    Version => "-v",
    SubcommandVersion => "version",
    CapitalVersion => "-V",
);

generate_argument!(
    HelpArg,
    // Prefer `--help` before `-h` for the same reason as VersionArg.
    LongHelp => "--help",
    Help => "-h",
    SubcommandHelp => "help",
    CapitalHelp => "-H",
);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    generate_argument!(
        Test,
        One => "one",
        Two => "two",
        Three => "three",
    );

    #[test]
    fn test_enum_generation() {
        assert_eq!(
            vec!["one", "two", "three"],
            Test::variants()
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_version_and_help_prefer_long_forms() {
        assert_eq!(
            vec!["--version", "-v", "version", "-V"],
            VersionArg::variants()
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec!["--help", "-h", "help", "-H"],
            HelpArg::variants()
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
        );
    }
}
