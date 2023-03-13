use std::{error::Error, fs::File, io::BufReader};
use std::io::{self, BufRead};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    bytes: bool,
    lines: bool,
    words: bool,
}

pub struct FileInfo {
    lines: usize,
    words: usize,
    bytes: usize,
}

pub fn get_flags() ->  MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("z")
        .arg(
            Arg::with_name("files")
            .default_value("-")
            .multiple(true),    
        )
        .arg(
            Arg::with_name("bytes")
                .short("c"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l"),
        )
        .arg(
            Arg::with_name("words")
                .short("w"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let bytes = matches.is_present("bytes");
    let lines = matches.is_present("lines");
    let words = matches.is_present("words");

    Ok(Config {
        files, bytes, lines, words,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename{
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count<T: BufRead>(file: &mut T) -> MyResult<FileInfo> {
    let mut lines: usize = 0;
    let mut words: usize = 0;
    let mut bytes: usize = 0;
    loop {
        let mut line = String::new();
        let b = file.read_line(&mut line)?;
        if b == 0 {
            break;
        }
        lines += 1;
        words += line.split_whitespace().count();
        bytes += b;
    }
    Ok(FileInfo { lines, words, bytes})
}

fn print(config: &Config, file_info: &FileInfo, filename: &str) {
    if config.lines {
        print!("{:<6}", file_info.lines);
    }
    if config.words {
        print!("{:<6}", file_info.words);
    }
    if config.bytes {
        print!("{:<6}", file_info.bytes);
    }
    if !config.lines && !config.words && !config.bytes {
        print!("{:<6}{:<6}{:<6}", file_info.lines, file_info.words, file_info.bytes);
    }
    println!("{}", filename);
}

pub fn run(config: &Config) -> MyResult<()> {
    let mut total:FileInfo = FileInfo { lines: 0, words: 0, bytes: 0 };
    let num_file = config.files.len();
    // for ... in ... 
    // for statement will move ownership out
    // use iter() to get ref
    for filename in &config.files {
        // can use &String when &str is expected
        if let Ok(mut file) = open(filename) {
            let file_info = count(&mut file)?;
            print(&config, &file_info, &filename);
            total.lines += file_info.lines;
            total.words += file_info.words;
            total.bytes += file_info.bytes;
        } else {
            println!("Open {} filed!", filename);
        }
    }
    if num_file > 1 {
        print(&config, &total, "total");
        // why can the following sentence can be compiled
        // print(&config, &total, &"total");
    }
    Ok(())
}