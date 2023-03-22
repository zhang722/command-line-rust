use std::{error::Error};
use std::io::{self, BufRead, BufReader, Read};
use std::fs::File;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// tailr
/// -c bytes,
/// -n lines,
/// -q quiet-mode
#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub bytes: Option<PlusUsize>,
    pub lines: Option<PlusUsize>,
    pub quiet: bool,
}

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PlusUsize {
    Usize(usize),
    PlusUsize(usize),
}

pub fn get_flags() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("z")
        .arg(
            Arg::with_name("files")
                .default_value("-")
                .multiple(true),    
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .takes_value(true)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let bytes = matches.value_of("bytes")
        .map(|f| parse_plus_usize(f)
            .map_err(|e| println!("{}", e)).unwrap()
        );
    let lines = matches.value_of("lines")
        .map(|f| parse_plus_usize(f)
        .map_err(|e| println!("{}", e)).unwrap());
    let quiet = matches.is_present("quiet");

    Ok(Config { 
        files, bytes, lines, quiet
    })
}


pub fn parse_plus_usize(input: &str) -> MyResult<PlusUsize> {
    match input.parse::<usize>() {
        Ok(num) => {
            if input.starts_with('+') {
                Ok(PlusUsize::PlusUsize(num))
            } else {
                Ok(PlusUsize::Usize(num))
            }
        },
        Err(_) => {Err(From::from("parse failed"))},
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: &Config) -> MyResult<()> {
    let files_count = config.files.len();
    for filename in config.files.iter() {
        if files_count > 1 && !config.quiet {
            println!("===>{}<===", filename);
        }
        let file = open(filename)?;
        if let Some(num) = &config.lines {
            let lines = file.lines();
            match num {
                PlusUsize::PlusUsize(num) => {
                    let lines = lines.skip(*num);
                    for line in lines {
                        println!("{}", line.unwrap());
                    }
                },
                PlusUsize::Usize(num) => {
                    let lines_can_rev = lines.collect::<Vec<_>>();
                    let lines = lines_can_rev
                        .into_iter()
                        .rev()
                        .take(*num)
                        .rev();
                    for line in lines {
                        println!("{}", line.unwrap());
                    }
                },
            }
        } else if let Some(num) = &config.bytes {
            let bytes = file
                .bytes()
                .flatten()
                .collect::<Vec<_>>();
            match num {
                PlusUsize::PlusUsize(num) => {
                    let idx = if *num > bytes.len() {bytes.len()} else {*num};
                    print!("{}", String::from_utf8_lossy(&bytes[idx ..]));
                },
                PlusUsize::Usize(num) => {
                    let idx = if *num > bytes.len() {0} else {*num};
                    print!("{}", String::from_utf8_lossy(&bytes[idx ..]));
                },
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::parse_plus_usize;
    use crate::PlusUsize;

    #[test]
    fn test_parse_plus_usize() {
        let a = "+3";
        let b = parse_plus_usize(a);
        assert_eq!(b.unwrap(), PlusUsize::PlusUsize(3));

        let a = "3";
        let b = parse_plus_usize(a);
        assert_eq!(b.unwrap(), PlusUsize::Usize(3));

        let a = "-3";
        let b = parse_plus_usize(a);
        assert!(b.map_err(|err| println!("{}", err)).is_ok());
    }
}
