use std::path::PathBuf;
use clap::CommandFactory;
use directories::ProjectDirs;
use regex::Regex;
use tracing::{debug, info};
use crate::{BIN_NAME, BIN_NAME_O, CREDENTIALS_FILE_DEFAULT, PKG_NAME, PKG_NAME_O, PKG_REPOSITORY, PKG_REPOSITORY_O, URL_README, VERSION, VERSION_O};
use crate::args::Args;

pub fn get_credentials_default_path() -> PathBuf {
    let dir = ProjectDirs::from_path(PathBuf::from(get_prog_without_ext())).unwrap();
    // fs::create_dir_all(dir.data_dir());
    let dp = dir.data_dir().join(CREDENTIALS_FILE_DEFAULT);
    debug!(
        "Data will be put into project directory {:?}.",
        dir.data_dir()
    );
    info!("Credentials file with private key is {}.", dp.display());
    dp
}

/// Gets version number, static if available, otherwise default.
pub fn get_version() -> &'static str {
    VERSION_O.unwrap_or(VERSION)
}

/// Gets Rust package name, static if available, otherwise default.
pub fn get_pkg_name() -> &'static str {
    PKG_NAME_O.unwrap_or(PKG_NAME)
}

/// Gets Rust binary name, static if available, otherwise default.
pub fn get_bin_name() -> &'static str {
    BIN_NAME_O.unwrap_or(BIN_NAME)
}

/// Gets Rust package repository, static if available, otherwise default.
pub fn get_pkg_repository() -> &'static str {
    PKG_REPOSITORY_O.unwrap_or(PKG_REPOSITORY)
}

/// Gets program name without extension.
pub fn get_prog_without_ext() -> &'static str {
    get_bin_name() // with -rs suffix
    // get_pkg_name() // without -rs suffix
}

/// Prints the usage info
pub fn usage() {
    let help_str = Args::command().render_usage().to_string();
    println!("{}", &help_str);
    println!("Options:");
    let help_str = Args::command().render_help().to_string();
    let v: Vec<&str> = help_str.split('\n').collect();
    for l in v {
        if l.starts_with("  -") || l.starts_with("      --") {
            println!("{}", &l);
        }
    }
}

/// Prints the short help
pub fn help() {
    let help_str = Args::command().render_help().to_string();
    // println!("{}", &help_str);
    // regex to remove shortest pieces "Details:: ... \n  -"
    // regex to remove shortest pieces "Details:: ... \n      --"
    // regex to remove shortest pieces "Details:: ... \nPS:"
    // 2 regex groups: delete and keep.
    // [\S\s]*? ... match anything in a non-greedy fashion
    // stop when either "PS:", "  -" or "      --" is reached
    let re = Regex::new(r"(?P<del>[ ]+Details::[\S\s]*?)(?P<keep>\nPS:|\n  -|\n      --)").unwrap();
    let after = re.replace_all(&help_str, "$keep");
    print!("{}", &after.replace("\n\n", "\n")); // remove empty lines
    println!("{}", "Use --manual to get more detailed help information.");
}

/// Prints the long help
pub fn manual() {
    let help_str = Args::command().render_long_help().to_string();
    println!("{}", &help_str);
}

/// Prints the README.md file
pub async fn readme() {
    match reqwest::get(URL_README).await {
        Ok(resp) => {
            debug!("Got README.md file from URL {:?}.", URL_README);
            println!("{}", resp.text().await.unwrap())
        }
        Err(ref e) => {
            println!(
                "Error getting README.md from {:#?}. Reported error {:?}.",
                URL_README, e
            );
        }
    };
}
