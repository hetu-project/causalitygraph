use std::{env, error};
use std::path::PathBuf;

use clap::{ArgMatches, crate_description, crate_name};

use crate::cli::set_clap;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    //proto::build::set().expect("failed to make proto");

    // env::set_var("RUST_LOG", "debug");
    // loop1().await;
    let matches = set_clap(
        crate_name!(),
        crate_description!(),
    ).get_matches();

    process_cmd(&matches).expect("failed to process cmd");

    println!("init ok");
    // RT.block_on(async {
    //     // wait
    //     tokio::signal::ctrl_c().await.unwrap();
    // });
    loop {}
    Ok(())
}

fn process_cmd(matches: &ArgMatches<'_>) -> Result<(), Box<dyn error::Error>> {
    // to add handle cmd
    if let Some(f) = matches.value_of("log_path") {
        //zchronod_logger::init_zchronod_logger_with_path(f, &find_env("RUST_LOG"))
    } else {
        zchronod_logger::init_zhronod_log_with_default()
    }
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path = config_path.join("chronod.yaml");
    let config = matches.value_of("config").unwrap_or(config_path.to_str().unwrap());
    process::init_chrono_node(config);
    Ok(())
}

fn find_env(env: &str) -> String {
    if let Ok(path_value) = env::var(env) {
        path_value
    } else {
        String::new()
    }
}