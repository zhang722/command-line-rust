/// grepr
/// -i| --ignore-case
/// -v| --invert-match
/// -c| --count
/// -r| --recursive
/// files
use std::{
    error::Error, 
    io::{BufRead, BufReader, self}, 
    fs::File
};

use regex::Regex;
use clap::{App, Arg};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    ignore: bool,
    invert: bool,
    count: bool,
    recursive: bool,
}

pub fn get_flags() -> MyResult<Config> {
    let matches = App::new("grepr")
        .author("z")
        .version("0.1.0")
        .arg(
            Arg::with_name("pattern"),
        )
        .arg(
            Arg::with_name("files")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("ignore-case")
                .short("i")
                .long("ignore-case"),    
        )
        .arg(
            Arg::with_name("invert-match")
                .short("v")
                .long("invert-match"),    
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count"),    
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive"),    
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let pattern_string = matches.value_of("pattern").unwrap();
    let pattern = Regex::new(pattern_string).unwrap();
    let ignore= matches.is_present("ignore-case");
    let invert= matches.is_present("invert-match");
    let count = matches.is_present("count");
    let recursive = matches.is_present("recursive");

    Ok(Config {
        files, pattern, ignore, invert, count, recursive,
    })

}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn open_files(config: &Config) {
    for filename in &config.files {
        if let Ok(file) = open(filename) {
            for line in file.lines() {
                println!("{}:{}", filename, line.unwrap());
            }
        }
    }
}

mod test {

use std::fs;

use walkdir::{DirEntry, WalkDir};

#[test]
fn test_walk_dir() -> Result<(), ()> {
    let walker = WalkDir::new(".").into_iter();
    for entry in walker.filter_map(|f| f.ok()) {
        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    }
    Err(())
}

#[test]
fn test_fs_read_dir() -> Result<(), ()> {
    for file in fs::read_dir(".").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
    Err(())
}

}
