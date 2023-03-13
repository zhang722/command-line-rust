use std::error::Error;

use clap::{App, Arg};
use regex::Regex;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .author("z")
        .version("0.1.0")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .default_value(r"\*rc")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("type")
                .short("t")    
                .long("type")
                .takes_value(true)
                .possible_values(&["f", "d", "l"])
                .default_value("f")
                .multiple(true),
        )
        .arg(
            Arg::with_name("paths")
                .default_value(".")
                .multiple(true),
        )
        .get_matches();

    let paths = matches.values_of_lossy("paths").unwrap();

    let names = matches.values_of_lossy("name").unwrap()
        .into_iter()
        .map(|name| {
            Regex::new(&name).map_err(|_| format!("invalid name {}", name))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| {
            x.unwrap()
        })
        .collect();

    let entry_types = matches.values_of_lossy("type").unwrap()
        .iter()
        .map(|x| {
            match x.as_str() {
                "f" => EntryType::File,
                "d" => EntryType::Dir,
                "l" => EntryType::Link,
                _ => unreachable!("failed"),
            }
        })
        .collect();

    Ok(Config {
        paths, names, entry_types
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if config.entry_types.iter().any(|x| {
                        match x {
                            EntryType::Link => entry.file_type().is_symlink(),
                            EntryType::Dir => entry.file_type().is_dir(),
                            EntryType::File => entry.file_type().is_file(),
                        }
                    }) 
                    && config.names.iter().any(|x| {
                        x.is_match(&entry.path().display().to_string())
                    }) 
                    {
                        println!("{}", &entry.path().display());
                    }
                }
            }
        }
    } 
    Ok(())
}


mod test {

use regex::Regex;
use walkdir::WalkDir;

// struct MyError(String);
#[test]
fn test_walk_dir() -> Result<(), ()> {
    for entry in WalkDir::new(".") {
        println!("{}", entry.unwrap().path().display());
    }
    Ok(())
}

#[test]
fn test_regex() {
    let re = Regex::new(r"\\*rc").unwrap();
    assert!(re.is_match("src"));
}

#[test]
fn test_access_vec() {
    let s1 = String::from("1");
    let s2 = String::from("2");
    let s3 = String::from("3");
    let a = vec![Ok(s1), Ok(s2), Ok(s3)];
}

}