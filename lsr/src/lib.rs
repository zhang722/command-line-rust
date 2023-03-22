use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub entries: Vec<String>,
    pub show_hidden: bool,
    pub long: bool,
}

pub fn get_flags() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .author("z")
        .arg(
            Arg::with_name("entries")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("long")
                .short("l"),   
        )
        .arg(
            Arg::with_name("show-hidden")
                .short("a"),
        )
        .get_matches();
    let entries = matches.values_of_lossy("entries")
        .expect("Error: not correct dir");
    let show_hidden = matches.is_present("show-hidden");
    let long = matches.is_present("long");

    Ok(Config { entries, show_hidden, long})
}

pub fn run() -> MyResult<()> {
    Ok(())
}
