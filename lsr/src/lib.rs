use std::error::Error;
use std::fs;
use std::time::SystemTime;

use chrono::Datelike;
use clap::{App, Arg};
use tabular::{Table, Row};
use chrono::{NaiveDateTime};

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

pub fn run(config: &Config) -> MyResult<()> {
    for entry in &config.entries {
        let entries = fs::read_dir(entry)?;
        let mut table = Table::new("{:<}  {:<}  {:<}");
        for entry in entries {
            let path = entry?.path();
            if let Some(path) = path.to_str() {
                let mut show: bool = false;

#[cfg(target_os="windows")]
                if config.show_hidden || !path.starts_with(r".\."){
                    show = true;
                } 

#[cfg(target_os="linux")] 
                if config.show_hidden || !path.starts_with(r"."){
                    show = true;
                }

                if show {
                    let modified = fs::metadata(path)?.modified()?;
                    let dt: NaiveDateTime = NaiveDateTime::from_timestamp_opt(
                        modified.duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i64,
                        0,
                    ).unwrap();
                    
                    if config.long {
                        table.add_row(Row::new()
                            .with_cell(format!("{}/{}/{}", dt.year(), dt.month(), dt.day()))
                            .with_cell(dt.time())
                            .with_cell(path));
                    } else {
                        println!("{}", path);
                    }
                }

            }
            
        }
        println!("{}", table);
    }
    
    Ok(())
}
