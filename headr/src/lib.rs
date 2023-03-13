use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub line: usize,
    pub bytes: Option<usize>,
}

pub fn get_flags() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .arg(
            Arg::with_name("lines")
                .short("n")    
                .long("lines")
                .value_name("LINES")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")    
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("LINES")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches.value_of("lines")
        .map(parse_positive_int)
        .transpose()?;

    let bytes = matches.value_of("bytes")
        .map(parse_positive_int)
        .transpose()?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        line: lines.unwrap(),
        bytes: bytes,
    })
}

fn parse_positive_int(s: &str) -> MyResult<usize> {
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(s)),
    }
}

fn open(filename: &str) -> std::io::Result<File> {
    let file = File::open(filename)?;
    Ok(file)
}

pub fn run(config: Config) -> MyResult<()> {
    let file_num = config.files.len();

    for filename in config.files {
        match open(&filename) {
            Err(_) => (),
            Ok(file) => {
                if file_num > 1 {
                    println!("==> {} <==", filename);
                }
                match config.bytes {
                    Some(n) => {
                        let mut handle = file.take(n as u64);
                        let mut buffer = vec![0; n];
                        let bytes_read = handle.read(&mut buffer)?;
                        println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                        println!("");
                    },
                    None => {
                        let f = BufReader::new(file);
                        for line in f.lines().take(config.line) {
                            println!("{}", line.unwrap());
                        }
                        println!("");
                    }
                }
            }
        }
    }
    Ok(())
}