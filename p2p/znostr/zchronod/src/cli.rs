use {
    clap::{App, Arg},
};

pub fn set_clap<'a>(name: &str, about: &'a str) -> App<'a, 'a> {
    App::new(name)
        .about(about)
        .arg(
            Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("FILEPATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use"),
        ).
        arg(
            Arg::with_name("log_path")
                .short("l")
                .long("log")
                .value_name("LOGPATH")
                .takes_value(true)
                .global(true)
                .help("log path to store log file"),
        )
}