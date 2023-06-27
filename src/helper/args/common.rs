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
    Version => "-v",
    CapitalVersion => "-V",
    LongVersion => "--version",
    SubcommandVersion => "version",
);

generate_argument!(
    HelpArg,
    Help => "-h",
    LongHelp => "--help",
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
}
