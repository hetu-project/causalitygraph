use std::fmt;
use std::str::FromStr;
use clap::ValueEnum;

/// Enumerator used for --output option
#[derive(Clone, Debug, Copy, PartialEq, Default, ValueEnum)]
pub enum Output {
    // None: only useful if one needs to know if option was used or not.
    // Sort of like an or instead of an Option<Sync>.
    // We do not need to know if user used the option or not,
    // we just need to know the value.
    /// Text: Indicates to print human readable text, default
    #[default]
    Text,
    /// Json: Indicates to print output in Json format
    Json,
    /// Json Max: Indicates to to print the maximum anount of output in Json format
    JsonMax,
    /// Json Spec: Indicates to to print output in Json format, but only data that is according to Nostr Specifications
    JsonSpec,
}

/// is_ functions for the enum
impl Output {
    pub fn is_text(&self) -> bool {
        self == &Self::Text
    }

    // pub fn is_json_spec(&self) -> bool { self == &Self::JsonSpec }
}

/// Converting from String to Listen for --listen option
impl FromStr for Output {
    type Err = ();
    fn from_str(src: &str) -> Result<Output, ()> {
        return match src.to_lowercase().replace('-', "_").trim() {
            "text" => Ok(Output::Text),
            "json" => Ok(Output::Json),
            "jsonmax" | "json_max" => Ok(Output::JsonMax), // accept all 3: jsonmax, json-max, json_max
            "jsonspec" | "json_spec" => Ok(Output::JsonSpec),
            _ => Err(()),
        };
    }
}

/// Creates .to_string() for Listen for --listen option
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}