use std::{error::Error};
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
    repeated: bool,
    ignore: bool,
    skip_chars: Option<usize>,
    unique: bool,
    check_chars: Option<usize>,
}

pub trait ConditionalEq {
    type Other;
    fn eq(&self, other: &Self::Other, config: &Config) -> bool;
}

impl ConditionalEq for String {
    type Other = String;

    fn eq(&self, other: &Self::Other, config: &Config) -> bool {
        let mut this_string = String::from(self);
        let mut other_string = String::from(other);

        if let Some(n) = config.check_chars {
            this_string.replace_range(if n < this_string.len() {n} else {this_string.len()}.., "");
            other_string.replace_range(if n < other_string.len() {n} else {other_string.len()}.., "");
        }
    
        if let Some(n) = config.skip_chars {
            this_string.replace_range(0.. if n >= this_string.len() {this_string.len()} else {n}, "");
            other_string.replace_range(0.. if n >= other_string.len() {other_string.len()} else {n}, "");
        } 

        if config.ignore {
            this_string = this_string.to_lowercase();
            other_string = other_string.to_lowercase();
        }
        this_string == other_string
    }
}

pub fn get_flags() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .author("z")
        .version("0.1.0")
        .arg(
            Arg::with_name("in_file")
                .takes_value(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("c")
                .short("c")
                .long("count"),
        )
        .arg(
            Arg::with_name("d")
                .short("d")
                .long("repeated"),
        )
        .arg(
            Arg::with_name("f")
                .short("f")
                .long("skip-fields")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("i")
                .short("i")
                .long("ignore-case"),
        )
        .arg(
            Arg::with_name("s")
                .short("s")
                .long("skip-chars")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("u")
                .short("u")
                .long("unique"),
        )
        .arg(
            Arg::with_name("w")
                .short("w")
                .long("check-chars")
                .takes_value(true),
        )
        .get_matches();
    let in_file = matches.value_of("in_file").unwrap().to_string();
    let out_file = matches.value_of("out_file").map(String::from);
    let count = matches.is_present("c");
    let r = matches.is_present("d");
    let ignore= matches.is_present("i");
    let skip_chars = matches.value_of("s").map(parse_positive_int).transpose()?;
    let u= matches.is_present("u");
    let check_chars = matches.value_of("w").map(parse_positive_int).transpose()?;

    let repeated = if r && u || !r && !u {true} else {r};
    let unique = if r && u || !r && !u {true} else {u};


    Ok(Config {
        in_file, out_file, count, repeated, ignore, skip_chars, unique, check_chars
    })
}

fn parse_positive_int(x: &str) -> MyResult<usize> {
    match x.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from("parse positive int failed")),    
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))) 
    }
}

fn print(file: &mut Box<dyn Write>, count: usize, line: &str, config: &Config) {
    match (config.repeated, config.unique, config.count, count > 1) {
        (true, true, true, _) |
        (true, false, true, true) |
        (false, true, true, false) => write!(file, "{} {}\n", count, line),

        (true, true, false, _) | 
        (true, false, false, true) | 
        (false, true, false, false) => write!(file, "{}\n", line),
        _ => Ok(()),
    }.unwrap();
}

pub fn run(config: &Config) -> MyResult<()> {
    let file = open(&config.in_file)?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(filename) => Box::new(File::create(filename)?),
        _ => Box::new(io::stdout()),
    };

    let mut lines = file.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut last_line = first_line;
    let mut count = 1;
    for line in lines.map(|l| l.unwrap()) {
        if ConditionalEq::eq(&line, &last_line, &config){
            count += 1;
        } else { // reach a different line
            // println!("{}  {}", count, last_line);
            print(&mut out_file, count, &last_line, &config);

            count = 1;
            last_line = line;
        }
    }
    // println!("{}  {}", count, last_line);
    print(&mut out_file, count, &last_line, &config);
    
    Ok(())
}
