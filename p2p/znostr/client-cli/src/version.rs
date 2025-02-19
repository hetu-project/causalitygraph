use std::fmt;
use std::str::FromStr;
use clap::ValueEnum;

/// Enumerator used for --version option
#[derive(Clone, Debug, Copy, PartialEq, Default, ValueEnum)]
pub(crate) enum Version {
    /// Check if there is a newer version available
    #[default]
    Check,
}

/// is_ functions for the enum
// impl Version {
//     pub fn is_check(&self) -> bool {
//         self == &Self::Check
//     }
// }

/// Converting from String to Version for --version option
impl FromStr for Version {
    type Err = ();
    fn from_str(src: &str) -> Result<Version, ()> {
        return match src.to_lowercase().trim() {
            "check" => Ok(Version::Check),
            _ => Err(()),
        };
    }
}

/// Creates .to_string() for Sync for --sync option
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}