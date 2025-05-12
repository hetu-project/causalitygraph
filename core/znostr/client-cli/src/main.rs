//! Welcome to nostr-commander!
//!
//! nostr-commander is a simple terminal-based CLI client of
//! Nostr <https://github.com/nostr-protocol>. It lets you create a
//! Nostr user, subscribe and follow posts of other
//! users and send encrypted, private DMs to your Nostr friends.
//!
//! Please help improve the code and add features  :pray:  :clap:
//!
//! Usage:
//! - run `nostr-commander-rs --help`
//! - run `nostr-commander-rs --manual`
//! - run `nostr-commander-rs --readme`
//!
//! For more information, see read the README.md
//! <https://github.com/8go/nostr-commander-rs/blob/main/README.md>
//! file.

#![allow(dead_code)] // crate-level allow  // Todo
#![allow(unused_variables)] // Todo
#![allow(unused_imports)] // Todo

use std::env;
use std::fmt::{self, Debug};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::panic;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use atty::Stream;
// use mime::Mime;
use chrono::Utc;
// use bitcoin_hashes::sha256::Hash;
use clap::{ColorChoice, CommandFactory, Parser, ValueEnum};
use directories::ProjectDirs;
use nostr_sdk::{bitcoin::hashes::sha256::Hash, Client, Contact, EventBuilder, EventId, nostr::event::kind::Kind, nostr::event::tag::TagKind, nostr::key::Keys, nostr::key::XOnlyPublicKey, nostr::message::ClientMessage, nostr::message::relay::RelayMessage, nostr::message::subscription::Filter, nostr::Metadata, nostr::nips::nip3041, nostr::prelude::core::time::Duration, nostr::types::time::Timestamp, nostr::UncheckedUrl, prelude::{FromBech32, FromSkStr, ToBech32}, relay::RelayPoolNotification, RelayPoolNotification::{Event, Message, RelayStatus, Shutdown, Stop}, Url};
use nostr_sdk::nips::nip3041::{PollData, VoteData};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;
use tracing::{debug, enabled, error, info, Level, trace, warn};
use tracing_subscriber;
use update_informer::{Check, registry};

use args::Args;
use credential::Credentials;
use error::Error;
use log::LogLevel;
use output::Output;
use relay::Relay;
use version::Version;

mod error;
mod version;
mod log;
mod output;
mod args;
mod relay;
mod credential;
mod util;


// /// import nostr-sdk Client related code of general kind: create_user, delete_user, etc // todo
// mod client; // todo
// use crate::client::dummy; // todo

/// the version number from Cargo.toml at compile time
const VERSION_O: Option<&str> = option_env!("CARGO_PKG_VERSION");
/// fallback if static compile time value is None
const VERSION: &str = "unknown version";
/// the package name from Cargo.toml at compile time, usually nostr-commander
const PKG_NAME_O: Option<&str> = option_env!("CARGO_PKG_NAME");
/// fallback if static compile time value is None
const PKG_NAME: &str = "nostr-commander";
/// the name of binary program from Cargo.toml at compile time, usually nostr-commander-rs
const BIN_NAME_O: Option<&str> = option_env!("CARGO_BIN_NAME");
/// fallback if static compile time value is None
const BIN_NAME: &str = "nostr-commander-rs";
/// he repo name from Cargo.toml at compile time,
/// e.g. string `https://github.com/8go/nostr-commander-rs/`
const PKG_REPOSITORY_O: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");
/// fallback if static compile time value is None
const PKG_REPOSITORY: &str = "https://github.com/8go/nostr-commander-rs/";
/// default name for login credentials JSON file
const CREDENTIALS_FILE_DEFAULT: &str = "credentials.json";
// /// default timeouts for waiting for the Nostr server, in seconds
// const TIMEOUT_DEFAULT: u64 = 60;
/// default POW difficulty
const POW_DIFFICULTY_DEFAULT: u8 = 20;
/// URL for README.md file downloaded for --readme
const URL_README: &str = "https://raw.githubusercontent.com/8go/nostr-commander-rs/main/README.md";


/// A struct for the credentials. These will be serialized into JSON
/// and written to the credentials.json file for permanent storage and
/// future access.

/// Gets the *default* path (including file name) of the credentials file
/// The default path might not be the actual path as it can be overwritten with command line
/// options.


/// Gets the *actual* path (including file name) of the credentials file
/// The default path might not be the actual path as it can be overwritten with command line
/// options.
fn get_credentials_actual_path(ap: &Args) -> &PathBuf {
    &ap.credentials
}

/// Return true if credentials file exists, false otherwise
fn credentials_exist(ap: &Args) -> bool {
    let dp = util::get_credentials_default_path();
    let ap = get_credentials_actual_path(ap);
    debug!(
        "credentials_default_path = {:?}, credentials_actual_path = {:?}",
        dp, ap
    );
    let exists = ap.is_file();
    if exists {
        debug!("{:?} exists and is file. Not sure if readable though.", ap);
    } else {
        debug!("{:?} does not exist or is not a file.", ap);
    }
    exists
}


/// Prints the version information
pub fn version(ap: &Args) {
    if ap.output.is_text() {
        println!();
        println!(
            "  _|      _|      _|_|_|                     {}",
            util::get_prog_without_ext()
        );
        print!("  _|_|    _|    _|             _~^~^~_       ");
        println!("a Nostr CLI client written in Rust");
        println!();
    } else {
        print_json(
            &json!({
                "program": util::get_prog_without_ext(),
                "repo": util::get_pkg_repository(),
                "version": util::get_version(),
                "icon": "NC :crab:",
            }),
            ap.output,
            0,
            "",
        );
    }
}

/// Prints the installed version and the latest crates.io-available version
pub fn version_check(ap: &Args) {
    let key1 = "Installed version";
    let value1 = util::get_version();
    let key2: &str;
    let value2: String;
    let key3: &str;
    let value3: String;
    let name = env!("CARGO_PKG_NAME");
    let location: String = "https://crates.io/crates/".to_owned() + name;
    let version = env!("CARGO_PKG_VERSION");
    let informer = update_informer::new(registry::Crates, name, version).check_version();
    match informer {
        Ok(Some(version)) => {
            key2 = "New available version";
            value2 = format!("{:?}", version);
            key3 = "New version available at";
            value3 = location;
        }
        Ok(None) => {
            key2 = "Status";
            value2 = "You are up-to-date. You already have the latest version.".to_owned();
            key3 = "Update required";
            value3 = "No".to_owned();
        }
        Err(ref e) => {
            key2 = "Error";
            value2 = "Could not get latest version.".to_owned();
            key3 = "Error message";
            value3 = format!("{:?}", e);
        }
    };
    print_json(
        &json!({
            key1: value1,
            key2: value2,
            key3: value3,
        }),
        ap.output,
        0,
        "",
    );
}

/// Asks the public for help
pub fn contribute(ap: &Args) {
    let text = format!(
        "{}{}{}{}{}{}",
        "This project is currently an experiment. ",
        "If you know Rust and are interested in Nostr, please have a look at the repo ",
        util::get_pkg_repository(),
        ". Please contribute code to improve the ",
        util::get_prog_without_ext(),
        " Nostr CLI client. Safe!"
    );
    print_json(
        &json!({
            "Contribute": text,
        }),
        ap.output,
        0,
        "",
    );
}

/// Reads metadata item from keyboard and puts it into the Args.
fn get_name(ap: &mut Args) {
    print!("Enter an optional name for this Nostr account (e.g. John Doe): ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Name left empty. That is okay!");
            ap.creds.metadata.name = None;
        }
        _ => {
            ap.creds.metadata.name = Some(input.trim().to_owned());
            info!("Name set to {:?}.", ap.creds.metadata.name);
        }
    }
}

/// Reads metadata item from keyboard and puts it into the Args.
fn get_display_name(ap: &mut Args) {
    print!("Enter an optional display name for this Nostr account (e.g. Jonnie): ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Display name left empty. That is okay!");
            ap.creds.metadata.display_name = None;
        }
        _ => {
            ap.creds.metadata.display_name = Some(input.trim().to_owned());
            info!("Display_name set to {:?}.", ap.creds.metadata.display_name);
        }
    }
}

/// Reads metadata item from keyboard and puts it into the Args.
fn get_about(ap: &mut Args) {
    print!(
        "Enter an optional description for this Nostr account (e.g. nostr loving surfing dude): "
    );
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("About left empty. That is okay!");
            ap.creds.metadata.about = None;
        }
        _ => {
            ap.creds.metadata.about = Some(input.trim().to_owned());
            info!("About set to {:?}.", ap.creds.metadata.about);
        }
    }
}

/// Reads metadata item from keyboard and puts it into the Args.
fn get_picture(ap: &mut Args) {
    let mut repeat = true;
    while repeat {
        repeat = false;
        print!("Enter an optional picture for this Nostr account (e.g. 'https://example.com/avatar.png' or 'file://./somedir/localfile.png'): ");
        std::io::stdout()
            .flush()
            .expect("error: could not flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        match input.trim() {
            "" => {
                info!("Picture left empty. That is okay!");
                ap.creds.metadata.picture = None;
            }
            _ => match Url::parse(input.trim()) {
                Ok(url) => {
                    ap.creds.metadata.picture = Some(url.as_str().to_string());
                    info!("Picture set to {:?}.", ap.creds.metadata.picture);
                }
                Err(ref e) => {
                    error!(
                        "{:?} is not a valid URL. Try again or leave empty. Reported error is {:?}.",
                        input.trim(), e
                    );
                    repeat = true;
                }
            },
        }
    }
}

/// Reads metadata item from keyboard and puts it into the Args.
fn get_nip05(ap: &mut Args) {
    print!("Enter an optional nip05 name for this Nostr account (e.g. john@example.com): ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Nip05 left empty. That is okay!");
            ap.creds.metadata.nip05 = None;
        }
        _ => {
            ap.creds.metadata.nip05 = Some(input.trim().to_owned());
            info!("Nip05 set to {:?}.", ap.creds.metadata.about);
        }
    }
}

/// Reads metadata item from keyboard.
fn get_proxy() -> Option<SocketAddr> {
    loop {
        print!("Enter proxy for relay (e.g. https://127.0.0.1:9050) or leave empty for no proxy: ");
        std::io::stdout()
            .flush()
            .expect("error: could not flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        match input.trim() {
            "" => {
                info!("Proxy left empty. That is okay!");
                return None;
            }
            _ => match SocketAddr::from_str(input.trim()) {
                Ok(u) => {
                    info!("proxy {:?} is accepted.", &u);
                    return Some(u);
                }
                Err(ref e) => {
                    error!(
                        "{:?} is not a valid proxy. Try again or leave empty. Reported error is {:?}.",
                        input.trim(), e
                    );
                }
            },
        }
    }
}

/// Reads metadata item from keyboard and puts it into the Args.
fn get_relays(ap: &mut Args) {
    println!("Enter one or multiple optional relays for this Nostr account.");
    let mut repeat = true;
    while repeat {
        print!("Enter relay name (e.g. wss://relay.example.com) or leave empty to move on: ");
        std::io::stdout()
            .flush()
            .expect("error: could not flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        match input.trim() {
            "" => {
                info!("Relay left empty. That is okay!");
                repeat = false;
            }
            _ => match Url::parse(input.trim()) {
                Ok(u) => {
                    if u.scheme() == "wss" || u.scheme() == "ws" {
                        let proxy = crate::get_proxy();
                        ap.creds.relays.push(Relay::new(u.clone(), proxy));
                        info!("relay {:?} {:?} added.", &u, proxy);
                    } else {
                        error!(
                        "{:?} is not a valid URL. Scheme is not 'wss'. Try again or leave empty.",
                        input.trim(),
                    );
                    }
                }
                Err(ref e) => {
                    error!(
                        "{:?} is not a valid URL. Try again or leave empty. Reported error is {:?}.",
                        input.trim(), e
                    );
                }
            },
        }
    }
}

/// Read credentials from disk
pub(crate) fn read_credentials(ap: &mut Args) -> Result<(), Error> {
    match Credentials::load(get_credentials_actual_path(&ap)) {
        Ok(c) => {
            info!(
                "Successfully loaded credentials from credentials file {:?}.",
                get_credentials_actual_path(&ap)
            );
            ap.creds = c;
            return Ok(());
        }
        Err(ref e) => {
            error!(
                "Error: failed to read credentials file {:?}. Aborting. Correct path? Error reported: {:?}.",
                get_credentials_actual_path(&ap),
                e,
            );
            return Err(Error::StorageFailure);
        }
    }
}

/// is this syntactically a valid relay string?
pub(crate) fn is_relay_str(relay: &str) -> bool {
    match Url::parse(relay) {
        Ok(r) => {
            if r.scheme() == "wss" {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

/// is this syntactically a valid relay string?
pub(crate) fn is_relay_url(relay: &Url) -> bool {
    if relay.scheme() != "wss" && relay.scheme() != "ws" {
        return false;
    } else if relay.host_str().is_none() {
        return false;
    }
    true
}

/// Handle the --create_user CLI argument
pub(crate) fn cli_create_user(ap: &mut Args) -> Result<(), Error> {
    if !ap.create_user {
        return Err(Error::UnsupportedCliParameter);
    }
    if credentials_exist(ap) {
        error!(concat!(
            "Credentials file already exists. You have already created a user in ",
            "the past. No --create-user is needed. Aborting. If you really want to create ",
            "a new user, then delete the current user first with '--delete-user', or move ",
            "or remove credentials file manually. ",
            "Or just run your command again but without the '--create-user' option to ",
            "use the currently existing user. ",
        ));
        return Err(Error::UserAlreadyExists);
    }
    match ap.name.as_ref() {
        None => {
            get_name(ap); // read from kb, put into metadata
        }
        Some(n) => {
            if n.trim().is_empty() {
                ap.creds.metadata.name = None;
            } else {
                ap.creds.metadata.name = Some(n.trim().to_owned());
            }
        }
    }
    match ap.display_name.as_ref() {
        None => {
            get_display_name(ap); // read from kb, put into metadata
        }
        Some(n) => {
            if n.trim().is_empty() {
                ap.creds.metadata.display_name = None;
            } else {
                ap.creds.metadata.display_name = Some(n.trim().to_owned());
            }
        }
    }
    match ap.about.as_ref() {
        None => {
            get_about(ap); // read from kb, put into metadata
        }
        Some(n) => {
            if n.trim().is_empty() {
                ap.creds.metadata.about = None;
            } else {
                ap.creds.metadata.about = Some(n.trim().to_owned());
            }
        }
    }
    match ap.picture.clone() {
        None => {
            get_picture(ap); // read from kb, put into metadata
        }
        Some(n) => {
            if (n.scheme() == "none" || n.scheme() == "file")
                && (n.path() == "/" || n.path() == "")
                && n.host().is_none()
            {
                ap.creds.metadata.picture = None;
            } else {
                ap.creds.metadata.picture = Some(n.to_string());
            }
        }
    }
    match ap.nip05.as_ref() {
        None => {
            get_nip05(ap); // read from kb, put into metadata
        }
        Some(n) => {
            if n.trim().is_empty() {
                ap.creds.metadata.nip05 = None;
            } else {
                ap.creds.metadata.nip05 = Some(n.trim().to_owned());
            }
        }
    }
    info!("Metadata is: {:?}", ap.creds.metadata);

    if ap.add_relay.is_empty() {
        get_relays(ap);
    } else {
        let num = ap.add_relay.len();
        let mut i = 0;
        while i < num {
            if is_relay_url(&ap.add_relay[i]) {
                ap.creds
                    .relays
                    .push(Relay::new(ap.add_relay[i].clone(), ap.proxy));
            } else {
                error!(
                    "Invalid relay syntax for relay {:?}. Skipping it.",
                    ap.add_relay[i]
                )
            }
            i += 1;
        }
    }
    ap.creds.relays.dedup_by(|a, b| a.url == b.url);

    // Generate new keys
    let my_keys: Keys = Keys::generate();
    debug!("Generated private key is: {:?}", my_keys.secret_key());
    debug!("Generated public  key is: {:?}", my_keys.public_key());
    match my_keys.public_key().to_bech32() {
        Ok(k) => ap.creds.public_key_bech32 = k,
        Err(ref e) => {
            error!(
                "Error: failed to convert public key. Aborting. Error reported: {:?}. ({:?})",
                e, my_keys
            );
            return Err(Error::KeyFailure);
        }
    }
    match my_keys.secret_key()?.to_bech32() {
        Ok(k) => ap.creds.secret_key_bech32 = k,
        Err(ref e) => {
            error!(
                "Error: failed to convert private key. Aborting. Error reported: {:?}. ({:?})",
                e, my_keys
            );
            return Err(Error::KeyFailure);
        }
    }
    match ap.creds.save(get_credentials_actual_path(&ap)) {
        Ok(()) => {
            info!("Successfully stored credentials in credentials file {:?}. Protect it, it contains your private key. Data stored is {:?}.", get_credentials_actual_path(&ap), &ap.creds);
        }
        Err(ref e) => {
            error!(
                "Error: failed to store credentials. Aborting. Error reported: {:?}. ({:?})",
                e, my_keys
            );
            return Err(Error::StorageFailure);
        }
    }
    Ok(())
}


pub async fn cli_publish_poll(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let mut multi_select = false;

    print!("Enter multi / single : ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Left empty. Default single");
        }
        "multi" => {
            multi_select = true;
        }
        "single" => {
            multi_select = false;
        }
        _ => {
            error!(
                "Error: must input multi / single"
            );
            return Err(Error::KeyFailure);
        }
    }

    let mut title = "";

    print!("Enter a title : ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Left empty");
        }
        _ => {
            title = input.trim();
        }
    }

    let mut info = "";

    print!("Enter info : ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Left empty");
        }
        _ => {
            info = input.trim();
        }
    }

    let mut options = Vec::new();

    print!("Enter options ex.(op1/op2/op3...) : ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Left empty");
        }
        _ => {
            options = input.trim().split("/").map(|op| op.to_string()).collect();
        }
    }

    let my_keys = Keys::from_sk_str(&ap.creds.secret_key_bech32)?;
    let poll = PollData::new(multi_select, "0", &title, &info, &options);
    let poll_event: nostr_sdk::Event = EventBuilder::build_poll(poll.clone()).to_event(&my_keys).expect("REASON");
    match client.send_event(poll_event).await {
        Ok(ref event_id) => print!(
            "Event_id {:?}",
            event_id
        ),
        Err(ref e) => {
            error!("Publish fail {:?}", e);
        }
    }
    Ok(())
}


pub async fn cli_make_vote(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let mut event_id = "";

    print!("Enter event id : ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Left empty");
        }
        _ => {
            event_id = input.trim();
        }
    }

    let mut choices = Vec::new();

    print!("Enter choices ex.(0/1/2...) : ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            error!(
                "Error: must have choice"
            );
            return Err(Error::KeyFailure);
        }
        _ => {
            choices = input.trim().split("/").map(|op| op.to_string()).collect();
        }
    }

    let mut reason = "";

    print!("Enter vote reason : ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Left empty");
        }
        _ => {
            reason = input.trim();
        }
    }

    let my_keys = Keys::from_sk_str(&ap.creds.secret_key_bech32)?;
    let event_id = EventId::from_hex(event_id).expect("REASON");
    let vote = VoteData::new(event_id, &choices, (&reason).to_string());
    let vote_event: nostr_sdk::Event = EventBuilder::build_vote(vote.clone()).to_event(&my_keys).expect("REASON");
    match client.send_event(vote_event).await {
        Ok(ref event_id) => print!(
            "Event_id {:?}",
            event_id
        ),
        Err(ref e) => {
            error!("Vote fail {:?}", e);
        }
    }
    Ok(())
}

pub async fn cli_query_poll(client: &Client) -> Result<(), Error> {
    let mut event_id = "";

    print!("Enter event id : ");
    std::io::stdout()
        .flush()
        .expect("error: could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    match input.trim() {
        "" => {
            info!("Left empty");
        }
        _ => {
            event_id = input.trim();
        }
    }
    let event_id = EventId::from_hex(event_id).expect("REASON");
    let msg = ClientMessage::Query { specific_sid: event_id };
    match client.send_msg(msg).await {
        Ok(ref event_id) => print!(
            "Event_id {:?}",
            event_id
        ),
        Err(ref e) => {
            error!("Vote fail {:?}", e);
        }
    }
    Ok(())
}

pub async fn cli_get_sids(client: &Client) -> Result<(), Error> {
    let msg = ClientMessage::Query_SID;
    match client.send_msg(msg).await {
        Ok(ref event_id) => print!(
            "Event_id {:?}",
            event_id
        ),
        Err(ref e) => {
            error!("Vote fail {:?}", e);
        }
    }
    Ok(())
}


/// Add relays to from Credentials to client
pub(crate) async fn add_relays_from_creds(client: &mut Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0u32;
    let num = ap.creds.relays.len();
    let mut i = 0;
    while i < num {
        let relay = ap.creds.relays[i].clone();
        match client.add_relay(relay.url.as_str()).await {
            Ok(true) => {
                debug!(
                    "add_relay with relay {:?} with proxy {:?} successful.",
                    relay.url, relay.proxy
                );
            }
            Ok(false) => {}
            Err(ref e) => {
                error!(
                    "Error: add_relay() returned error. Relay {:?} with proxy {:?} not added. Reported error {:?}.",
                    relay.url, relay.proxy, e
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::AddRelayFailed)
    } else {
        Ok(())
    }
}

/// Handle the --add_relay CLI argument.
/// Add relays from --add-relay.
pub(crate) async fn cli_add_relay(client: &mut Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0u32;
    let num = ap.add_relay.len();
    let mut i = 0;
    while i < num {
        if is_relay_url(&ap.add_relay[i]) {
            match client.add_relay(ap.add_relay[i].as_str()).await {
                Ok(true) => {
                    debug!(
                        "add_relay with relay {:?} and proxy {:?} successful.",
                        ap.add_relay[i], ap.proxy
                    );
                    ap.creds
                        .relays
                        .push(Relay::new(ap.add_relay[i].clone(), ap.proxy));
                }
                Ok(false) => {}
                Err(ref e) => {
                    error!(
                    "Error: add_relay() returned error. Relay {:?} not added. Reported error {:?}.",
                    ap.add_relay[i], e
                );
                    err_count += 1;
                }
            }
        } else {
            error!(
                "Error: Relay {:?} is syntactically not correct. Relay not added.",
                ap.add_relay[i],
            );
            err_count += 1;
        }
        i += 1;
    }
    ap.creds.relays.dedup_by(|a, b| a.url == b.url);
    match ap.creds.save(get_credentials_actual_path(ap)) {
        Ok(()) => {
            debug!(
                "writing new relays {:?} to credentials file successful.",
                ap.creds.relays
            );
        }
        Err(ref e) => {
            error!(
                "Error: writing new relays {:?} to credentials file failed. Reported error {:?}.",
                ap.creds.relays, e
            );
            err_count += 1;
        }
    }
    if err_count != 0 {
        Err(Error::AddRelayFailed)
    } else {
        Ok(())
    }
}

/// Handle the --remove-relay CLI argument, remove CLI args contacts from creds data structure
pub(crate) async fn cli_remove_relay(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let num = ap.remove_relay.len();
    let mut i = 0;
    while i < num {
        ap.creds.relays.retain(|x| x.url != ap.remove_relay[i]);
        i += 1;
    }
    Ok(())
}

fn trim_newline(s: &mut String) -> &mut String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    return s;
}

/// Handle the --publish CLI argument
/// Publish notes.
pub(crate) async fn cli_publish(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.publish.len();
    let mut i = 0;
    while i < num {
        let note = &ap.publish[i];
        trace!("publish: {:?}", note);
        if note.is_empty() {
            info!("Skipping empty text note.");
            i += 1;
            continue;
        };
        if note == "--" {
            info!("Skipping '--' text note as these are used to separate arguments.");
            i += 1;
            continue;
        };
        // - map to - (stdin pipe)
        // \- maps to text r'-', a 1-letter message
        let fnote = if note == r"-" {
            let mut line = String::new();
            if atty::is(Stream::Stdin) {
                print!("Message: ");
                std::io::stdout()
                    .flush()
                    .expect("error: could not flush stdout");
                io::stdin().read_line(&mut line)?;
            } else {
                io::stdin().read_to_string(&mut line)?;
            }
            line
        } else if note == r"_" {
            let mut eof = false;
            while !eof {
                let mut line = String::new();
                match io::stdin().read_line(&mut line) {
                    // If this function returns Ok(0), the stream has reached EOF.
                    Ok(n) => {
                        if n == 0 {
                            eof = true;
                            debug!("Reached EOF of pipe stream.");
                        } else {
                            debug!(
                                "Read {n} bytes containing \"{}\\n\" from pipe stream.",
                                trim_newline(&mut line.clone())
                            );
                            // match client.publish_text_note(&line, &[]).await {
                            //     Ok(ref event_id) => debug!(
                            //         "Publish_text_note number {:?} from pipe stream sent successfully. {:?}. event_id {:?}",
                            //         i, &line, event_id
                            //     ),
                            //     Err(ref e) => {
                            //         err_count += 1;
                            //         error!(
                            //             "Publish_text_note number {:?} from pipe stream failed. {:?}",
                            //             i, &line
                            //         );
                            //     }
                            // }
                        }
                    }
                    Err(ref e) => {
                        err_count += 1;
                        error!("Error: reading from pipe stream reported {}", e);
                    }
                }
            }
            "".to_owned()
        } else if note == r"\-" {
            "-".to_string()
        } else if note == r"\_" {
            "_".to_string()
        } else if note == r"\-\-" {
            "--".to_string()
        } else if note == r"\-\-\-" {
            "---".to_string()
        } else {
            note.to_string()
        };
        if fnote.is_empty() {
            info!("Skipping empty text note.");
            i += 1;
            continue;
        }

        // match client.publish_text_note(&fnote, &[]).await {
        //     Ok(ref event_id) => debug!(
        //         "Publish_text_note number {:?} sent successfully. {:?}, event_id {:?}",
        //         i, &fnote, event_id
        //     ),
        //     Err(ref e) => {
        //         err_count += 1;
        //         error!("Publish_text_note number {:?} failed. {:?}", i, &fnote);
        //     }
        // }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::PublishFailed)
    } else {
        Ok(())
    }
}

/// Publish DMs.
pub(crate) async fn send_dms(
    client: &Client,
    notes: &[String],
    recipient: &Keys,
) -> Result<(), Error> {
    trace!("send_dms: {:?} {:?}", notes, recipient);
    let mut err_count = 0usize;
    let num = notes.len();
    let mut i = 0;
    while i < num {
        let note = &notes[i];
        trace!("send_dms: {:?}", note);
        if note.is_empty() {
            info!("Skipping empty text note.");
            i += 1;
            continue;
        };
        if note == "--" {
            info!("Skipping '--' text note as these are used to separate arguments.");
            i += 1;
            continue;
        };
        // - map to - (stdin pipe)
        // \- maps to text r'-', a 1-letter message
        let fnote = if note == r"-" {
            let mut line = String::new();
            if atty::is(Stream::Stdin) {
                print!("Message: ");
                std::io::stdout()
                    .flush()
                    .expect("error: could not flush stdout");
                io::stdin().read_line(&mut line)?;
            } else {
                io::stdin().read_to_string(&mut line)?;
            }
            line
        } else if note == r"_" {
            let mut eof = false;
            while !eof {
                let mut line = String::new();
                match io::stdin().read_line(&mut line) {
                    // If this function returns Ok(0), the stream has reached EOF.
                    Ok(n) => {
                        if n == 0 {
                            eof = true;
                            debug!("Reached EOF of pipe stream.");
                        } else {
                            debug!(
                                "Read {n} bytes containing \"{}\\n\" from pipe stream.",
                                trim_newline(&mut line.clone())
                            );
                            match client.send_direct_msg(recipient.public_key(), &line, None).await {
                                Ok(event_id) => debug!(
                                    "send_direct_msg number {:?} from pipe stream sent successfully. {:?}, sent to {:?}, event_id {:?}",
                                    i, &line, recipient.public_key(), event_id
                                ),
                                Err(ref e) => {
                                    err_count += 1;
                                    error!(
                                        "send_direct_msg number {:?} from pipe stream failed. {:?}, sent to {:?}",
                                        i, &line, recipient.public_key()
                                    );
                                }
                            }
                        }
                    }
                    Err(ref e) => {
                        err_count += 1;
                        error!("Error: reading from pipe stream reported {}", e);
                    }
                }
            }
            "".to_owned()
        } else if note == r"\-" {
            "-".to_string()
        } else if note == r"\_" {
            "_".to_string()
        } else if note == r"\-\-" {
            "--".to_string()
        } else if note == r"\-\-\-" {
            "---".to_string()
        } else {
            note.to_string()
        };
        if fnote.is_empty() {
            info!("Skipping empty text note.");
            i += 1;
            continue;
        }

        match client
            .send_direct_msg(recipient.public_key(), &fnote, None)
            .await
        {
            Ok(ref event_id) => debug!(
                "DM message number {:?} sent successfully. {:?}, sent to {:?}, event_id {:?}.",
                i,
                &fnote,
                recipient.public_key(),
                event_id
            ),
            Err(ref e) => {
                err_count += 1;
                error!(
                    "DM message number {:?} failed. {:?}, sent to {:?}.",
                    i,
                    &fnote,
                    recipient.public_key()
                );
            }
        }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::PublishPowFailed)
    } else {
        Ok(())
    }
}

/// Handle the --dm CLI argument
/// Publish DMs.
pub(crate) async fn cli_dm(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let num = ap.dm.len();
    if num < 2 {
        return Err(Error::MissingCliParameter);
    }
    match cstr_to_pubkey(ap, ap.dm[0].trim()) {
        Ok(pk) => {
            let keys = Keys::from_public_key(pk);
            let notes = &ap.dm[1..];
            send_dms(client, notes, &keys).await
        }
        Err(ref e) => {
            error!(
                "Error: Not a valid key. Cannot send this DM. Aborting. Key {:?}, 1st Msg {:?} ",
                ap.dm[0].trim(),
                ap.dm[1]
            );
            Err(Error::InvalidKey)
        }
    }
}

/// Send messages to one channel.


/// Handle the --send-channel-message CLI argument
/// Publish messages to one channel.


/// Is key in subscribed_authors list?
pub(crate) fn is_subscribed_author(ap: &Args, pkey: &XOnlyPublicKey) -> bool {
    ap.creds.subscribed_authors.contains(pkey)
}

/// Get contact for given alias.
/// Returns None if alias does not exist in contact list.
pub(crate) fn get_contact_by_alias(ap: &Args, alias: &str) -> Option<Contact> {
    ap.creds
        .contacts
        .iter()
        .find(|s| s.alias == Some(alias.to_string()))
        .cloned()
}

/// Get contact for given pubkey.
/// Returns None if pubkey does not exist in contact list.
pub(crate) fn get_contact_by_key(ap: &Args, pkey: XOnlyPublicKey) -> Option<Contact> {
    ap.creds.contacts.iter().find(|s| s.pk == pkey).cloned()
}

/// Get contact alias for given pubkey, or if not in contacts return given pubkey.
/// Returns alias if contact with this pubkey exists.
/// Returns input pubkey if no contact with this pubkey exists.
pub(crate) fn get_contact_alias_or_keystr_by_key(ap: &Args, pkey: XOnlyPublicKey) -> String {
    match get_contact_by_key(ap, pkey) {
        Some(c) => match c.alias {
            Some(a) => a,
            None => pkey.to_string(),
        },
        None => pkey.to_string(),
    }
}

/// Get contact alias for given pubkey, or if not in contacts return None.
/// Returns Some(alias) if contact with this pubkey exists.
/// Returns None if no contact with this pubkey exists.
pub(crate) fn get_contact_alias_by_key(ap: &Args, pkey: XOnlyPublicKey) -> Option<String> {
    match get_contact_by_key(ap, pkey) {
        Some(c) => c.alias,
        None => None,
    }
}

/// Get contact alias for given pubkey string (string of XOnlyPublicKey), or if not in contacts return given pubkey.
/// Returns alias if contact with this pubkey exists.
/// Returns input pubkey if no contact with this pubkey exists.
pub(crate) fn get_contact_alias_or_keystr_by_keystr(ap: &Args, pkeystr: &str) -> String {
    match XOnlyPublicKey::from_str(pkeystr) {
        Ok(pkey) => match get_contact_by_key(ap, pkey) {
            Some(c) => match c.alias {
                Some(a) => a,
                None => pkey.to_string(),
            },
            None => pkey.to_string(),
        },
        Err(_) => pkeystr.to_string(),
    }
}

/// Get contact alias for given pubkey string (string of XOnlyPublicKey), or if not in contacts return None.
/// Returns Some(alias) if contact with this pubkey exists.
/// Returns None if no contact with this pubkey exists.
pub(crate) fn get_contact_alias_by_keystr(ap: &Args, pkeystr: &str) -> Option<String> {
    match XOnlyPublicKey::from_str(pkeystr) {
        Ok(pkey) => match get_contact_by_key(ap, pkey) {
            Some(c) => c.alias,
            None => None,
        },
        Err(_) => None,
    }
}

/// Handle the --add-contact CLI argument, write contacts from CLI args into creds data structure
pub(crate) async fn cli_add_contact(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let anum = ap.alias.len();
    let knum = ap.key.len();
    let rnum = ap.relay.len();
    if (anum != knum) || (anum != rnum) || (knum != rnum) {
        error!(
            "--alias, --key, and --relay must have the same amount of entries. {:?} {:?} {:?} ",
            anum, knum, rnum
        );
        return Err(Error::MissingCliParameter);
    }
    let mut i = 0;
    while i < anum {
        if ap.alias[i].trim().is_empty() {
            error!("Invalid user alias. Cannot be empty. Skipping this contact.");
            err_count += 1;
            i += 1;
            continue;
        }
        if get_contact_by_alias(ap, ap.alias[i].trim()).is_some() {
            error!("Invalid user alias. Alias already exists. Alias must be unique. Skipping this contact.");
            err_count += 1;
            i += 1;
            continue;
        }
        if !is_relay_url(&ap.relay[i]) {
            error!(
                "Relay {:?} is not valid. Skipping this contact.",
                ap.relay[i]
            );
            err_count += 1;
            i += 1;
            continue;
        }
        let key = &ap.key[i];
        match str_to_pubkey(key) {
            Ok(pkey) => {
                debug!("Valid key for contact. Key {:?}, {:?}.", key, pkey);
                let rurl = ap.relay[i].clone();
                ap.creds.contacts.push(Contact::new(
                    pkey,
                    Some(UncheckedUrl::from(rurl)),
                    Some(ap.alias[i].trim().to_string()),
                ));
                debug!("Added contact. Key {:?}, {:?}.", key, pkey);
            }
            Err(ref e) => {
                error!("Error: Invalid key {:?}. Skipping this contact.", key);
                err_count += 1;
                i += 1;
                continue;
            }
        }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::SubscriptionFailed)
    } else {
        Ok(())
    }
}

/// Handle the --remove-contact CLI argument, remove CLI args contacts from creds data structure
pub(crate) async fn cli_remove_contact(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let num = ap.alias.len();
    let mut i = 0;
    while i < num {
        ap.creds
            .contacts
            .retain(|x| x.alias != Some(ap.alias[i].trim().to_string()));
        i += 1;
    }
    Ok(())
}

/// Convert npub1... Bech32 key or Hex key or contact alias into a XOnlyPublicKey
/// Returns Error if neither valid Bech32, nor Hex key, nor contact alias.
pub(crate) fn cstr_to_pubkey(ap: &Args, s: &str) -> Result<XOnlyPublicKey, Error> {
    match get_contact_by_alias(ap, s) {
        Some(c) => Ok(c.pk),
        None => str_to_pubkey(s),
    }
}

/// Convert npub1... Bech32 key or Hex key into a XOnlyPublicKey
/// Returns Error if neither valid Bech32 nor Hex key.
pub(crate) fn str_to_pubkey(s: &str) -> Result<XOnlyPublicKey, Error> {
    match XOnlyPublicKey::from_bech32(s) {
        Ok(pkey) => {
            debug!(
                "Valid key in Bech32 format: Npub {:?}, Hex {:?}",
                s,
                pkey.to_bech32().unwrap() // public_key
            );
            return Ok(pkey);
        }
        Err(ref e) => match XOnlyPublicKey::from_str(s) {
            Ok(pkey) => {
                debug!(
                    "Valid key in Hex format: Hex {:?}, Npub {:?}",
                    s,
                    pkey.to_bech32().unwrap()
                );
                return Ok(pkey);
            }
            Err(ref e) => {
                error!("Error: Invalid key {:?}. Reported error: {:?}.", s, e);
                return Err(Error::InvalidKey);
            }
        },
    }
}

/// Convert npub1... Bech32 key or Hex key into a npub+hex pair as Vector.
/// s ... input, npub ... output, hex ... output.
/// Returns Error if neither valid Bech32 nor Hex key.
pub(crate) fn str_to_pubkeys(s: &str) -> Result<(String, String), Error> {
    match XOnlyPublicKey::from_bech32(s) {
        Ok(pkey) => {
            debug!(
                "Valid key in Bech32 format: Npub {:?}, Hex {:?}",
                s,
                pkey.to_string() // public_key
            );
            let npub = s.to_owned();
            // todo: zzz not sure about this
            let hex = pkey.to_string(); // pkey.to_bech32().unwrap();
            return Ok((npub, hex));
        }
        Err(ref e) => match XOnlyPublicKey::from_str(s) {
            Ok(pkey) => {
                debug!(
                    "Valid key in Hex format: Hex {:?}, Npub {:?}",
                    s,
                    pkey.to_bech32().unwrap()
                );
                let npub = pkey.to_bech32().unwrap();
                let hex = s.to_owned();
                return Ok((npub, hex));
            }
            Err(ref e) => {
                error!("Error: Invalid key {:?}. Reported error: {:?}.", s, e);
                return Err(Error::InvalidKey);
            }
        },
    }
}

/// Handle the --cli_npub_to_hex CLI argument
pub(crate) fn cli_npub_to_hex(ap: &Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.npub_to_hex.len();
    let mut i = 0;
    while i < num {
        match str_to_pubkeys(&ap.npub_to_hex[i]) {
            Ok((npub, hex)) => {
                debug!("Valid key. Npub {:?}, Hex: {:?}.", &npub, &hex);
                print_json(
                    &json!({
                        "npub": npub,
                        "hex": hex,
                    }),
                    ap.output,
                    0,
                    "",
                );
            }
            Err(ref e) => {
                error!(
                    "Error: Invalid key {:?}. Not added to subscription filter.",
                    &ap.npub_to_hex[i]
                );
                print_json(
                    &json!({
                        "npub": ap.npub_to_hex[i],
                        "error": "invalid key",
                    }),
                    ap.output,
                    0,
                    "",
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::ConversionFailed)
    } else {
        Ok(())
    }
}

/// Handle the --cli_hex_to_npub CLI argument
pub(crate) fn cli_hex_to_npub(ap: &Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.hex_to_npub.len();
    let mut i = 0;
    while i < num {
        match str_to_pubkeys(&ap.hex_to_npub[i]) {
            Ok((npub, hex)) => {
                debug!("Valid key. Npub {:?}, Hex: {:?}.", &npub, &hex);
                print_json(
                    &json!({
                        "npub": npub,
                        "hex": hex,
                    }),
                    ap.output,
                    0,
                    "",
                );
            }
            Err(ref e) => {
                error!(
                    "Error: Invalid key {:?}. Not added to subscription filter.",
                    &ap.hex_to_npub[i]
                );
                print_json(
                    &json!({
                        "hex": ap.hex_to_npub[i],
                        "error": "invalid key",
                    }),
                    ap.output,
                    0,
                    "",
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::ConversionFailed)
    } else {
        Ok(())
    }
}

/// Handle the --cli_get_pubkey_entity CLI argument
// pub(crate) async fn cli_get_pubkey_entity(client: &Client, ap: &mut Args) -> Result<(), Error> {
//     let mut err_count = 0usize;
//     let num = ap.get_pubkey_entity.len();
//     let mut i = 0;
//     while i < num {
//         match str_to_pubkey(&ap.get_pubkey_entity[i]) {
//             Ok(pkey) => {
//                 debug!(
//                     "Valid key. Key {:?}, Hex: {:?}.",
//                     &ap.subscribe_pubkey[i],
//                     pkey.to_string()
//                 );
//                 // no timeout
//                 match client.get_entity_of(pkey.to_string(), None).await {
//                     Ok(entity) => {
//                         debug!(
//                             "Valid key. Key {:?}, Hex: {:?}, Entity: {:?}",
//                             &ap.subscribe_pubkey[i],
//                             pkey.to_string(),
//                             entity
//                         );
//                         print_json(
//                             &json!({
//                                 "hex": pkey.to_string(),
//                                 "entity": entity, // prints as text like "Channel"
//                             }),
//                             ap.output,
//                             0,
//                             "",
//                         );
//                     }
//                     Err(ref e) => {
//                         debug!(
//                             "Valid key. Key {:?}, Hex: {:?}, Entity error: {:?}",
//                             &ap.subscribe_pubkey[i],
//                             pkey.to_string(),
//                             e
//                         );
//                         print_json(
//                             &json!({
//                                 "hex": pkey.to_string(),
//                                 "error": format!("{:?}",e),
//                             }),
//                             ap.output,
//                             0,
//                             "",
//                         );
//                     }
//                 }
//             }
//             Err(ref e) => {
//                 error!(
//                     "Error: Invalid key {:?}. No attempt made to determine entity.",
//                     &ap.get_pubkey_entity[i]
//                 );
//                 print_json(
//                     &json!({
//                         "key": ap.get_pubkey_entity[i],
//                         "error": "invalid key",
//                     }),
//                     ap.output,
//                     0,
//                     "",
//                 );
//                 err_count += 1;
//             }
//         }
//         i += 1;
//     }
//     if err_count != 0 {
//         Err(Error::GetEntityFailed)
//     } else {
//         Ok(())
//     }
// }

/// Handle the --subscribe-pubkey CLI argument, moving pkeys from CLI args into creds data structure
pub(crate) async fn cli_subscribe_pubkey(client: &mut Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.subscribe_pubkey.len();
    let mut pubkeys = Vec::new();
    let mut i = 0;
    while i < num {
        match str_to_pubkey(&ap.subscribe_pubkey[i]) {
            Ok(pkey) => {
                pubkeys.push(pkey);
                debug!(
                    "Valid key added to subscription filter. Key {:?}, Hex: {:?}.",
                    &ap.subscribe_pubkey[i],
                    pkey.to_string()
                );
            }
            Err(ref e) => {
                error!(
                    "Error: Invalid key {:?}. Not added to subscription filter.",
                    &ap.subscribe_pubkey[i]
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    ap.creds.subscribed_pubkeys.append(&mut pubkeys);
    ap.creds.subscribed_pubkeys.dedup_by(|a, b| a == b);
    if err_count != 0 {
        Err(Error::SubscriptionFailed)
    } else {
        Ok(())
    }
}

/// Handle the --unsubscribe-pubkey CLI argument, remove CLI args contacts from creds data structure
pub(crate) async fn cli_unsubscribe_pubkey(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.unsubscribe_pubkey.len();
    let mut i = 0;
    while i < num {
        match str_to_pubkey(&ap.unsubscribe_pubkey[i]) {
            Ok(pkey) => {
                ap.creds.subscribed_pubkeys.retain(|x| x != &pkey);
            }
            Err(ref e) => {
                error!(
                    "Error: Invalid key {:?}. Not removed from subscription filter.",
                    &ap.unsubscribe_pubkey[i]
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::UnsubscribeFailed)
    } else {
        Ok(())
    }
}

/// Handle the --subscribe-author CLI argument, moving authors from CLI args into creds data structure
pub(crate) async fn cli_subscribe_author(client: &mut Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.subscribe_author.len();
    let mut authors = Vec::new();
    let mut i = 0;
    while i < num {
        match str_to_pubkey(&ap.subscribe_author[i]) {
            Ok(pkey) => {
                authors.push(pkey);
                debug!(
                    "Valid key added to subscription filter. Key {:?}, {:?}.",
                    &ap.subscribe_author[i], pkey
                );
            }
            Err(ref e) => {
                error!(
                    "Error: Invalid key {:?}. Not added to subscription filter.",
                    &ap.subscribe_author[i]
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    ap.creds.subscribed_authors.append(&mut authors);
    ap.creds.subscribed_authors.dedup_by(|a, b| a == b);
    if err_count != 0 {
        Err(Error::SubscriptionFailed)
    } else {
        Ok(())
    }
}

/// Handle the --unsubscribe-author CLI argument, remove CLI args contacts from creds data structure
pub(crate) async fn cli_unsubscribe_author(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.unsubscribe_author.len();
    let mut i = 0;
    while i < num {
        match str_to_pubkey(&ap.unsubscribe_author[i]) {
            Ok(pkey) => {
                ap.creds.subscribed_authors.retain(|x| x != &pkey);
            }
            Err(ref e) => {
                error!(
                    "Error: Invalid key {:?}. Not removed from subscription filter.",
                    &ap.unsubscribe_author[i]
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::UnsubscribeFailed)
    } else {
        Ok(())
    }
}

/// Handle the --subscribe-channel CLI argument, moving pkeys from CLI args into creds data structure
pub(crate) async fn cli_subscribe_channel(client: &mut Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.subscribe_channel.len();
    let mut hashs = Vec::new();
    let mut i = 0;
    while i < num {
        match Hash::from_str(&ap.subscribe_channel[i]) {
            Ok(hash) => {
                hashs.push(hash);
                debug!(
                    "Valid key added to subscription filter. Key {:?}, Hash: {:?}.",
                    &ap.subscribe_channel[i],
                    hash.to_string()
                );
            }
            Err(ref e) => {
                error!(
                    "Error: Invalid key {:?}. Not added to subscription filter.",
                    &ap.subscribe_channel[i]
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    ap.creds.subscribed_channels.append(&mut hashs);
    ap.creds.subscribed_channels.dedup_by(|a, b| a == b);
    if err_count != 0 {
        Err(Error::SubscriptionFailed)
    } else {
        Ok(())
    }
}

/// Handle the --unsubscribe-channel CLI argument, remove CLI args contacts from creds data structure
pub(crate) async fn cli_unsubscribe_channel(client: &Client, ap: &mut Args) -> Result<(), Error> {
    let mut err_count = 0usize;
    let num = ap.unsubscribe_channel.len();
    let mut i = 0;
    while i < num {
        match Hash::from_str(&ap.unsubscribe_channel[i]) {
            Ok(hash) => {
                ap.creds.subscribed_channels.retain(|x| x != &hash);
                debug!(
                    "Valid key removed from subscription filter. Key {:?}, Hash: {:?}.",
                    &ap.unsubscribe_channel[i],
                    hash.to_string()
                );
            }
            Err(ref e) => {
                error!(
                    "Error: Invalid key {:?}. Not removed from subscription filter.",
                    &ap.unsubscribe_channel[i]
                );
                err_count += 1;
            }
        }
        i += 1;
    }
    if err_count != 0 {
        Err(Error::UnsubscribeFailed)
    } else {
        Ok(())
    }
}

/// Utility function to print JSON object as JSON or as plain text
/// depth: depth in nesting, on first call use 0.
// see https://github.com/serde-rs/json
pub(crate) fn print_json(jsonv: &Value, output: Output, depth: u32, separator: &str) {
    trace!("{:?}", jsonv);
    match output {
        Output::Text => {
            if depth != 0 {
                print!("    ");
            }
            if jsonv.is_object() {
                // if it is an object, check recursively
                for (key, val) in jsonv.as_object().unwrap() {
                    print!("{}:", key);
                    print_json(val, output, depth + 1, separator);
                    print!("    ");
                }
            } else if jsonv.is_boolean() {
                print!("{}", jsonv);
            } else if jsonv.is_null() {
                print!(""); // print nothing
            } else if jsonv.is_string() {
                print!("{}", jsonv);
            } else if jsonv.is_number() {
                print!("{}", jsonv);
            } else if jsonv.is_array() {
                print!("[ ");
                print!("{}", separator);
                let mut i = 0;
                while i < jsonv.as_array().unwrap().len() {
                    if i > 0 {
                        print!(",    ");
                    }
                    print_json(&jsonv[i], output, depth + 1, separator);
                    i += 1;
                    println!();
                }
                print!("{}", separator);
                print!(" ]");
            } else {
                debug!("not implemented type in print_json()");
                print!("{}", jsonv.to_string(), );
            }
            if depth == 0 {
                println!();
            }
        }
        Output::JsonSpec => (),
        _ => {
            // This can panic if output is piped and pipe is broken by receiving process
            println!("{}", jsonv.to_string(), );
        }
    }
}

/// Handle the --whoami CLI argument
pub(crate) fn cli_whoami(ap: &Args) -> Result<(), Error> {
    print_json(
        &json!({
            "name": ap.creds.metadata.name.clone(),
            "display_name": ap.creds.metadata.display_name.clone(),
        }),
        ap.output,
        0,
        "",
    );
    Ok(())
}

/// We need your code contributions! Please add features and make PRs! :pray: :clap:
#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut ap = Args::parse();
    let mut is_connected = false; // is this app connected to relays

    // eprintln!("Please delete user and create new user or edit your");
    // eprintln!("credentials file and make it look similar to the example");
    // eprintln!("given in the README.md file.");
    // eprintln!("");

    // handle log level and debug options
    let env_org_rust_log = env::var("RUST_LOG").unwrap_or_default().to_uppercase();
    // eprintln!("Original log_level option is {:?}", ap.log_level);
    // eprintln!("Original RUST_LOG is {:?}", &env_org_rust_log);


    if ap.show_cred_path {
        let dp = util::get_credentials_default_path();
        let ap = get_credentials_actual_path(&ap);
        println!("credentials_default_path = {:?}, credentials_actual_path = {:?}", dp, ap);
        return Ok(());
    }


    if ap.debug > 0 {
        // -d overwrites --log-level
        ap.log_level = LogLevel::Debug
    }
    if ap.log_level.is_none() {
        ap.log_level = LogLevel::from_str(&env_org_rust_log, true).unwrap_or(LogLevel::Error);
    }
    // overwrite environment variable, important because it might have been empty/unset
    env::set_var("RUST_LOG", ap.log_level.to_string());

    // set log level e.g. via RUST_LOG=DEBUG cargo run, use newly set venv var value
    // Send *all* output from Debug to Error to stderr
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(Level::from_str(&ap.log_level.to_string()).unwrap_or(Level::ERROR))
        .init();
    debug!("Original RUST_LOG env var is {}", env_org_rust_log);
    debug!(
        "Final RUST_LOG env var is {}",
        env::var("RUST_LOG").unwrap_or_default().to_uppercase()
    );
    debug!("Final log_level option is {:?}", ap.log_level);
    if enabled!(Level::TRACE) {
        debug!("Log level is set to TRACE.");
    } else if enabled!(Level::DEBUG) {
        debug!("Log level is set to DEBUG.");
    }
    debug!("Version is {}", util::get_version());
    debug!("Package name is {}", util::get_pkg_name());
    debug!("Repo is {}", util::get_pkg_repository());
    debug!("Arguments are {:?}", ap);

    match ap.version {
        None => (),                        // do nothing
        Some(None) => crate::version(&ap), // print version
        Some(Some(Version::Check)) => crate::version_check(&ap),
    }
    if ap.contribute {
        crate::contribute(&ap);
    };
    if ap.usage {
        util::usage();
        return Ok(());
    };
    if ap.help {
        util::help();
        return Ok(());
    };
    if ap.manual {
        util::manual();
        return Ok(());
    };
    if ap.readme {
        util::readme().await;
        return Ok(());
    };

    if ap.create_user {
        match crate::cli_create_user(&mut ap) {
            Ok(()) => {
                info!("User created successfully.");
            }
            Err(ref e) => {
                error!("Creating a user failed or credentials information could not be written to disk. Check your arguments and try --create-user again. Reported error is: {:?}", e);
                return Err(Error::CreatingUserFailed);
            }
        }
    } else {
        match crate::read_credentials(&mut ap) {
            Ok(()) => {
                info!("User created successfully.");
            }
            Err(ref e) => {
                error!("Credentials file does not exists or cannot be read. Try creating a user first with --create-user. Check your arguments and try again. Worst case if file is corrupted or lost, consider doing a '--delete-user' to clean up, then perform a new '--create-user'. {:?}.", e);
                return Err(Error::ReadingCredentialsFailed);
            }
        }
    }
    // credentials are filled now

    debug!("Welcome to nostr-commander-rs");

    let my_keys = Keys::from_sk_str(&ap.creds.secret_key_bech32)?;

    // Show public key
    if ap.show_public_key {
        print!(
            "Loaded public key in Nostr format is : {:?}",
            my_keys.public_key().to_string()
        );
        println!(
            "Loaded public key in Bech32 format is: {:?}",
            ap.creds.public_key_bech32
        );
    }
    // Show secret key
    if ap.show_secret_key {
        println!(
            "Loaded secret key in Nostr format is : {:?}",
            my_keys.secret_key()?.display_secret()
        );
        println!(
            "Loaded secret key in Bech32 format is: {:?}",
            ap.creds.secret_key_bech32
        );
    }
    // whoami
    if ap.whoami {
        cli_whoami(&ap)?;
    }
    // npub_to_hex
    if !ap.npub_to_hex.is_empty() {
        match cli_npub_to_hex(&ap) {
            Ok(()) => {
                info!("Converting keys from npub to hex successful.");
            }
            Err(ref e) => {
                error!(
                    "Converting keys from npub to hex failed. Reported error is: {:?}",
                    e
                );
            }
        }
    }
    // hex_to_npub
    if !ap.hex_to_npub.is_empty() {
        match cli_hex_to_npub(&ap) {
            Ok(()) => {
                info!("Converting keys from hex to npub successful.");
            }
            Err(ref e) => {
                error!(
                    "Converting keys from hex to npub failed. Reported error is: {:?}",
                    e
                );
            }
        }
    }
    // Create new client
    let mut client = Client::new(&my_keys);

    match add_relays_from_creds(&mut client, &mut ap).await {
        Ok(()) => {
            info!("Adding relays from credentials to client successful.");
        }
        Err(ref e) => {
            error!(
                "Adding relays from credentials to client failed. Reported error is: {:?}",
                e
            );
        }
    }
    // todo clean up code to separate better local action from client/remote action
    // Add relays, if --create-user the relays have already been added
    if !ap.add_relay.is_empty() && !ap.create_user {
        match crate::cli_add_relay(&mut client, &mut ap).await {
            Ok(()) => {
                info!("add_relay successful.");
            }
            Err(ref e) => {
                error!("add_relay failed. Reported error is: {:?}", e);
            }
        }
    }
    if !ap.remove_relay.is_empty() {
        match crate::cli_remove_relay(&client, &mut ap).await {
            Ok(()) => {
                info!("remove_relay successful.");
            }
            Err(ref e) => {
                error!("remove_relay failed. Reported error is: {:?}", e);
            }
        }
    }
    ap.creds.relays.dedup_by(|a, b| a.url == b.url);

    trace!("checking to see if it is necessary to call connect.");
    // todo: further optimize: --unsubscribe-... could remove subscriptions and make subscriptions empty,
    // but this is not yet checked.
    if ap.listen
        // || !ap.publish_pow.is_empty() // publish_pow_text_note discontinued since nostr-sdk v0.21.
        || !ap.publish.is_empty()
        || !ap.dm.is_empty()
        || !ap.send_channel_message.is_empty()
        || !ap.subscribe_pubkey.is_empty()
        || !ap.subscribe_author.is_empty()
        || !ap.subscribe_channel.is_empty()
        || !ap.get_pubkey_entity.is_empty()
    {
        // design decision: avoid connect_...()  call if no relay action is needed and everything can be done locally.
        // design decision: avoid connect...() if no client is needed.
        //
        // Do we need to connect on create-user ? No. --create-user just creates locally a key-pair.
        info!("initiating connect now.");
        client.connect().await;
        info!("connect successful.");
        is_connected = true;
    }

    if ap.create_user {
        // let metadata = Metadata::new()
        //     .name("username")
        //     .display_name("My Username")
        //     .about("Description")
        //     .picture(Url::from_str("https://example.com/avatar.png")?)
        //     .nip05("username@example.com");

        // Update profile metadata
        // client.update_profile() was removed from nostr-sdk API
    }

    // Set contacts, first in local file, second in client
    if ap.add_contact {
        match crate::cli_add_contact(&client, &mut ap).await {
            Ok(()) => {
                info!("add_contact successful.");
            }
            Err(ref e) => {
                error!("add_contact failed. Reported error is: {:?}", e);
            }
        }
    }
    if ap.remove_contact {
        match crate::cli_remove_contact(&client, &mut ap).await {
            Ok(()) => {
                info!("remove_contact successful.");
            }
            Err(ref e) => {
                error!("remove_contact failed. Reported error is: {:?}", e);
            }
        }
    }
    ap.creds.contacts.dedup_by(|a, b| a.alias == b.alias);
    if is_connected {
        trace!("setting contact list on server.");
        match client.set_contact_list(ap.creds.contacts.clone()).await {
            Ok(ref event_id) => {
                info!("set_contact_list successful. event_id {:?}", event_id);
            }
            Err(ref e) => {
                error!("set_contact_list failed. Reported error is: {:?}", e);
            }
        }
    } else {
        trace!("not setting contact list on server, because we are not connected.");
    }
    if ap.show_contacts {
        if ap.output.is_text() {
            for c in &ap.creds.contacts {
                print_json(&json!(c), ap.output, 0, "");
            }
        } else {
            print_json(&json!({"contacts": ap.creds.contacts}), ap.output, 0, "");
        }
    }
    // ap.creds.save(get_credentials_actual_path(&ap))?; // do it later

    // // Get pubkey entity
    // if !ap.get_pubkey_entity.is_empty() {
    //     match crate::cli_get_pubkey_entity(&client, &mut ap).await {
    //         Ok(()) => {
    //             info!("get_pubkey_entity successful.");
    //         }
    //         Err(ref e) => {
    //             error!("get_pubkey_entity failed. Reported error is: {:?}", e);
    //         }
    //     }
    // }

    trace!("checking if something needs to be published.");
    // Publish a text note
    if !ap.publish.is_empty() {
        match crate::cli_publish(&client, &mut ap).await {
            Ok(()) => {
                info!("publish successful.");
            }
            Err(ref e) => {
                error!("publish failed. Reported error is: {:?}", e);
            }
        }
    }
    // publish_pow_text_note discontinued since nostr-sdk v0.21.
    // // Publish a POW text note
    // if !ap.publish_pow.is_empty() {
    //     match crate::cli_publish_pow(&client, &mut ap).await {
    //         Ok(()) => {
    //             info!("publish_pow successful.");
    //         }
    //         Err(ref e) => {
    //             error!("publish_pow failed. Reported error is: {:?}", e);
    //         }
    //     }
    // }
    // Send DMs
    if !ap.dm.is_empty() {
        match crate::cli_dm(&client, &mut ap).await {
            Ok(()) => {
                info!("dm successful.");
            }
            Err(ref e) => {
                error!("dm failed. Reported error is: {:?}", e);
            }
        }
    }
    // Send channel messages


    // Subscribe keys
    if !ap.subscribe_pubkey.is_empty() {
        match crate::cli_subscribe_pubkey(&mut client, &mut ap).await {
            Ok(()) => {
                debug!("subscribe_pubkey successful. Subscriptions synchronized with credentials file.");
            }
            Err(ref e) => {
                error!("subscribe_pubkey failed. Reported error is: {:?}", e);
            }
        }
    }
    if !ap.creds.subscribed_pubkeys.is_empty() && ap.listen {
        let mut ksf: Filter;
        ksf = Filter::new().pubkeys(ap.creds.subscribed_pubkeys.clone());
        if ap.limit_number != 0 {
            ksf = ksf.limit(ap.limit_number);
        }
        if ap.limit_days != 0 {
            ksf = ksf.since(Timestamp::now() - Duration::new(ap.limit_days * 24 * 60 * 60, 0));
        }
        if ap.limit_hours != 0 {
            ksf = ksf.since(Timestamp::now() - Duration::new(ap.limit_hours * 60 * 60, 0));
        }
        if ap.limit_future_days != 0 {
            ksf =
                ksf.until(Timestamp::now() + Duration::new(ap.limit_future_days * 24 * 60 * 60, 0));
        }
        if ap.limit_future_hours != 0 {
            ksf = ksf.until(Timestamp::now() + Duration::new(ap.limit_future_hours * 60 * 60, 0));
        }
        info!("subscribe to keys initiated.");
        client.subscribe(vec![ksf]).await;
        info!("subscribe to keys successful.");
    }
    // Subscribe authors
    if !ap.subscribe_author.is_empty() {
        match crate::cli_subscribe_author(&mut client, &mut ap).await {
            Ok(()) => {
                debug!("subscribe_author successful. Subscriptions synchronized with credentials file.");
            }
            Err(ref e) => {
                error!("subscribe_author failed. Reported error is: {:?}", e);
            }
        }
    }
    if !ap.creds.subscribed_authors.is_empty() && ap.listen {
        let mut asf: Filter;
        asf = Filter::new();
        for author in &ap.creds.subscribed_authors {
            debug!("adding author {:?} to filter.", author);
            asf = asf.author(*author);
        }
        if ap.limit_number != 0 {
            asf = asf.limit(ap.limit_number);
        }
        if ap.limit_days != 0 {
            asf = asf.since(Timestamp::now() - Duration::new(ap.limit_days * 24 * 60 * 60, 0));
        }
        if ap.limit_hours != 0 {
            asf = asf.since(Timestamp::now() - Duration::new(ap.limit_hours * 60 * 60, 0));
        }
        if ap.limit_future_days != 0 {
            asf =
                asf.until(Timestamp::now() + Duration::new(ap.limit_future_days * 24 * 60 * 60, 0));
        }
        if ap.limit_future_hours != 0 {
            asf = asf.until(Timestamp::now() + Duration::new(ap.limit_future_hours * 60 * 60, 0));
        }
        info!("subscribe to authors initiated.");
        client.subscribe(vec![asf]).await;
        info!("subscribe to authors successful.");
    }
    // Subscribe channels
    if !ap.subscribe_channel.is_empty() {
        match crate::cli_subscribe_channel(&mut client, &mut ap).await {
            Ok(()) => {
                debug!("subscribe_channel successful. Subscriptions synchronized with credentials file.");
            }
            Err(ref e) => {
                error!("subscribe_channel failed. Reported error is: {:?}", e);
            }
        }
    }
    // Unsubscribe channels
    if !ap.unsubscribe_channel.is_empty() {
        match crate::cli_unsubscribe_channel(&mut client, &mut ap).await {
            Ok(()) => {
                debug!("unsubscribe_channel successful. Subscriptions synchronized with credentials file.");
            }
            Err(ref e) => {
                error!("unsubscribe_channel failed. Reported error is: {:?}", e);
            }
        }
    }
    if !ap.creds.subscribed_channels.is_empty() && ap.listen {
        let mut csf: Filter;
        let mut ev_vec: Vec<EventId> = Vec::new();
        for sc in &mut ap.creds.subscribed_channels {
            ev_vec.push(EventId::from(*sc));
        }

        csf = Filter::new().events(ev_vec);
        if ap.limit_number != 0 {
            csf = csf.limit(ap.limit_number);
        }
        if ap.limit_days != 0 {
            csf = csf.since(Timestamp::now() - Duration::new(ap.limit_days * 24 * 60 * 60, 0));
        }
        if ap.limit_hours != 0 {
            csf = csf.since(Timestamp::now() - Duration::new(ap.limit_hours * 60 * 60, 0));
        }
        if ap.limit_future_days != 0 {
            csf =
                csf.until(Timestamp::now() + Duration::new(ap.limit_future_days * 24 * 60 * 60, 0));
        }
        if ap.limit_future_hours != 0 {
            csf = csf.until(Timestamp::now() + Duration::new(ap.limit_future_hours * 60 * 60, 0));
        }
        info!("subscribe to channels initiated.");
        client.subscribe(vec![csf]).await;
        info!("subscribe to channels successful.");
    }
    ap.creds.save(get_credentials_actual_path(&ap))?;

    // notices will be published even if we do not go into handle_notification event loop
    // Design choice: Do not automatically listen when subscriptions exist, only listen to subscriptions if --listen is set.
    if ap.listen
    // || !ap.creds.subscribed_authors.is_empty()
    // || !ap.creds.subscribed_pubkeys.is_empty()
    {
        let mut num = ap.publish.len() + ap.dm.len() + ap.send_channel_message.len(); // + ap.publish_pow.len() // publish_pow_text_note discontinued since nostr-sdk v0.21.
        if ap.dm.len() > 1 {
            num -= 1; //adjust num, 1st arg of dm is key not msg
        }
        if ap.send_channel_message.len() > 1 {
            num -= 1; //adjust num, 1st arg of send_channel_message is key not msg
        }
        if num == 1 {
            info!(
                "You should be receiving {:?} 'OK' message with event id for the notice once it has been relayed.",
                num
            );
        } else if num > 1 {
            info!(
                "You should be receiving {:?} 'OK' messages with event ids, one for each notice that has been relayed.",
                num
            );
        }
        // Handle notifications
        match client
            .handle_notifications(|notification| async {
                debug!("Notification: {:?}", notification);
                match notification {
                    Stop => {
                        debug!("Stop: stopping");
                        // todo: zzz stopp
                    }
                    Shutdown => {
                        debug!("Shutdown: shutting down");
                        // todo: zzz shutdown
                    }
                    RelayStatus { relay_url: url, status: status } => {
                        debug!("Event-RelayStatus: url {:?}, relaystatus {:?}", url, status);
                    }
                    Event { relay_url: url, event: ev } => {
                        debug!("Event-Event: url {:?}, content {:?}, kind {:?}", url, ev.content, ev.kind);
                    }
                    Message { relay_url: url, message: msg } => {
                        // debug!("Message: {:?}", msg);
                        match msg {
                            RelayMessage::Closed { .. } => {}

                            RelayMessage::NegMsg { .. } => {}
                            RelayMessage::NegErr { .. } => {}

                            RelayMessage::Ok { event_id, status, message } => {
                                // Notification: ReceivedMessage(Ok { event_id: 123, status: true, message: "" })
                                // confirmation of notice having been relayed
                                info!("Message-OK: Notice, DM or message was relayed. Url is {:?}, Event id is {:?}. Status is {:?} and message is {:?}. You can investigate this event by looking it up on https://nostr.com/e/{}", url, event_id, status, message, event_id.to_string());
                                print_json(
                                    &json!({"event_type": "RelayMessage::Ok",
                                        "event_type_meaning": "Notice, DM or message was relayed successfully.",
                                        "event_id": event_id,
                                        "status": status,
                                        "message": message,
                                        "event_url": "https://nostr.com/e/".to_string() + &event_id.to_string(),
                                        "event_url_meaning": "You can investigate this event by looking up the event URL.",
                                    }),
                                    ap.output, 0, "",
                                );
                            }
                            RelayMessage::Notice { message } => {
                                debug!("Message-Notice: {:?}", message);
                            }
                            RelayMessage::Event { event, subscription_id } => {
                                // kind: Base(ChannelMessage) and Base(TextNote) and Base(Reaction)
                                let mut tags = "".to_owned();
                                let first = true;
                                for t in &event.tags {
                                    match t.kind() {
                                        TagKind::P => {
                                            trace!("tag vector: {:?}", t.as_vec());
                                            //match t.content() {
                                            //    Some(c) => {
                                            //        trace!("tag: {:?}", get_contact_alias_or_keystr_by_keystr(&ap, c));
                                            //        match get_contact_alias_by_keystr(&ap, c) {
                                            //            Some(a) => {
                                            //                if !first { tags += ", "; };
                                            //                tags += &a;
                                            //                first = false;
                                            //                },
                                            //            _ => ()
                                            //        }
                                            //    }
                                            //    None => ()
                                            //}
                                        }
                                        TagKind::E => info!("E message received. Not implemented."),  // todo!(),
                                        TagKind::Nonce => info!("Nonce message received. Not implemented."),  // todo!(),
                                        TagKind::Delegation => info!("Delegation message received. Not implemented."),  // todo!(),
                                        TagKind::ContentWarning => info!("ContentWarning message received. Not implemented."),  // todo!(),
                                        TagKind::Custom(_) => info!("Custom message received. Not implemented."),  // todo!(),
                                        _ => info!("Other message received. Not implemented."),  // todo!(),
                                    }
                                }
                                trace!("Message-Event: content {:?}, kind {:?}, from pubkey {:?}, with tags {:?}", event.content, event.kind, get_contact_alias_or_keystr_by_key(&ap, event.pubkey), event.tags);
                                let mut key_author = "key";
                                if is_subscribed_author(&ap, &event.pubkey) {
                                    key_author = "author";
                                    tags = get_contact_alias_or_keystr_by_key(&ap, event.pubkey);
                                };
                                match event.kind {
                                    Kind::ContactList => {
                                        debug!("Received Message-Event ContactList");
                                    }
                                    Kind::Reaction => {
                                        debug!("Received Message-Event Reaction: content {:?}", event.content);
                                    }
                                    Kind::TextNote => {
                                        info!("Subscription by {} ({}): content {:?}, kind {:?}, from pubkey {:?}", key_author, tags, event.content, event.kind, get_contact_alias_or_keystr_by_key(&ap, event.pubkey));
                                        print_json(
                                            &json!({
                                                "event_type": "RelayMessage::Event",
                                                "event_type_meaning": "Message was received because of subscription.",
                                                "subscribed_by": key_author,
                                                "author": get_contact_alias_or_keystr_by_key(&ap, event.pubkey),
                                                "content": event.content,
                                                "kind": event.kind, // writes integer like '1'
                                                "kind_text": format!("{:?}",event.kind), // writes text like "Base(TextNote)"
                                                "from_alias": get_contact_alias_or_keystr_by_key(&ap, event.pubkey),
                                                "from_pubkey": event.pubkey,
                                                "tags": tags
                                            }),
                                            ap.output, 0, "",
                                        );
                                    }
                                    Kind::ChannelMessage => {
                                        info!("Subscription by {} ({}): content {:?}, kind {:?}, from pubkey {:?}", key_author, tags, event.content, event.kind, get_contact_alias_or_keystr_by_key(&ap, event.pubkey));
                                        print_json(
                                            &json!({
                                                "event_type": "RelayMessage::Event",
                                                "event_type_meaning": "Message was received because of subscription.",
                                                "subscribed_by": key_author,
                                                "author": get_contact_alias_or_keystr_by_key(&ap, event.pubkey),
                                                "content": event.content,
                                                "kind": event.kind, // writes integer like '1'
                                                "kind_text": format!("{:?}",event.kind), // writes text like "Base(TextNote)"
                                                "from_alias": get_contact_alias_or_keystr_by_key(&ap, event.pubkey),
                                                "from_pubkey": event.pubkey,
                                                "tags": tags
                                            }),
                                            ap.output, 0, "",
                                        );
                                    }
                                    _ => ()
                                }
                            }
                            RelayMessage::EndOfStoredEvents(subscription_id) => {
                                debug!("Received Message-Event EndOfStoredEvents");
                            }
                            RelayMessage::Auth { challenge } => {
                                debug!("Received Message-Event Auth");
                            }
                            RelayMessage::Count { subscription_id, count } => {
                                debug!("Received Message-Event Count {:?}", count);
                            }
                        }
                    }
                }
                Ok(false)
            })
            .await
        {
            Ok(()) => {
                info!("handle_notifications successful.");
            }
            Err(ref e) => {
                error!("handle_notifications failed. Reported error is: {:?}", e);
            }
        }
    }

    if ap.publish_poll {
        match cli_publish_poll(&client, &mut ap).await {
            Ok(()) => {
                info!("Publish poll successfully.");
            }
            Err(ref e) => {
                error!("Publish poll fail. Reported error is: {:?}", e);
                return Err(Error::PublishPollFailed);
            }
        }
        return Ok(());
    }

    if ap.vote {
        match cli_make_vote(&client, &mut ap).await {
            Ok(()) => {
                info!("Make vote successfully.");
            }
            Err(ref e) => {
                error!("Make vote fail. Reported error is: {:?}", e);
                return Err(Error::MakeVoteFailed);
            }
        }
        return Ok(());
    }

    if ap.query_poll_state {
        match cli_query_poll(&client).await {
            Ok(()) => {
                info!("Make vote successfully.");
            }
            Err(ref e) => {
                error!("Make vote fail. Reported error is: {:?}", e);
                return Err(Error::MakeVoteFailed);
            }
        }
        return Ok(());
    }

    if ap.get_eids_poll {
        match cli_get_sids(&client).await {
            Ok(()) => {
                info!("Make vote successfully.");
            }
            Err(ref e) => {
                error!("Make vote fail. Reported error is: {:?}", e);
                return Err(Error::MakeVoteFailed);
            }
        }
        return Ok(());
    }


    debug!("Good bye");
    Ok(())
}

/// Future test cases will be put here
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_usage() {
        assert_eq!(util::usage(), ());
    }

    #[test]
    fn test_help() {
        assert_eq!(util::help(), ());
    }
}
