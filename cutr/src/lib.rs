use std::{error::Error, io::{self, BufRead, BufReader}, thread::current, fmt::Display, process::id, fs::File};

use clap::{App, Arg};
use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum EntryType {
    Byte,
    Char,
    Field,
}

#[derive(Debug)]
pub struct Config {
    file : String,
    indice: Vec<usize>,
    delim: String,
    entry_type: EntryType,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cutr")
        .author("z")
        .version("0.1.0")
        .arg(
            Arg::with_name("file")
                .default_value("-")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("b")
                .long("bytes")
                .conflicts_with("characters")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("characters")
                .short("c")    
                .long("characters")
                .conflicts_with("bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .conflicts_with("bytes")
                .conflicts_with("characters")
                .requires("fields")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fields")
                .short("f")
                .long("fields")
                .conflicts_with("bytes")
                .conflicts_with("characters")
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("file");
    let bytes = matches.value_of("bytes");
    let chars = matches.value_of("characters");
    let delim = matches.value_of("delimiter").map(|x| x.to_string()).unwrap_or("\t".to_string()) ;
    let fields = matches.value_of("fields");
    let entry_type: EntryType;
    let indice: Vec<usize>;

    if let Some(_) = bytes { 
        entry_type = EntryType::Byte;
        indice = bytes.map(|s| parse_list(s).unwrap()).unwrap();
    } 
    else if let Some(_) = chars { 
        entry_type = EntryType::Char;
        indice = chars.map(|s| parse_list(s).unwrap()).unwrap();
    }
    else { 
        entry_type = EntryType::Field;
        indice = fields.map(|s| parse_list(s).unwrap()).unwrap();
    }


    Ok(Config {
        file: file.unwrap().to_string(),
        indice,
        delim,
        entry_type,
    })
}

fn parse_usize(num: &str) -> MyResult<usize> {
    match num.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(num)),
    }
}


fn parse_list(list: &str) -> MyResult<Vec<usize>> {
    let re_comma_list = Regex::new(r"\d+(,\d+)+").unwrap();
    let re_dash_list = Regex::new(r"\d+-\d+").unwrap();
    if re_comma_list.is_match(list) {
        Ok(list.split(',')
            .map(|x| {
                x.parse::<usize>() 
                    .map_err(|e| println!("parse list failed: {}", e)
                )}
            .unwrap())
            .map(|x| x - 1)
            .collect::<Vec<usize>>())
    } else if re_dash_list.is_match(list) {
        let a = list.split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(a.len(), 2);

        Ok((a[0] - 1..=a[1] - 1).into_iter().collect())
    }
    else {
        list.parse::<usize>().map(|x| vec![x - 1]).map_err(|_| From::from("parse error"))
    }
}

pub fn run(config: Config) -> MyResult<()> {
    if let Ok(file) = open(&config.file) {
        let delim = match config.entry_type {
            EntryType::Field => Some(config.delim.chars().nth(0).unwrap()),
            _ => None,
        };
        for line in file.lines() {
            let l = line.unwrap();
            match config.entry_type {
                EntryType::Byte => {
                    let list = l.as_bytes();
                    control_print(&list, &config.indice, delim);
                }
                EntryType::Char => {
                    let list: Vec<char> = l.chars().collect();
                    control_print(&list, &config.indice, delim);
                }
                EntryType::Field => {
                    let list = l.split(delim.unwrap()).collect::<Vec<&str>>();
                    control_print(&list, &config.indice, delim);
                }
            }
        }
    } else {

    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn control_print<T: Display>(list: &[T], indice: &[usize], deli: Option<char>) {
    let len = list.len();
    let print = |idx: usize| {
        if idx < len {
            print!("{}", list[idx])
        }
    };
    let mut filter_iter = indice.iter().filter(|x| **x < len);
    let first = match filter_iter.next() {
        Some(first) => *first,
        None => return,
    };
    print(first);

    for idx in filter_iter {
        if let Some(c) = deli {
            print!("{}", c);
        }
        print(*idx);
    }
    print!("\n");
}


#[test]
fn test_parse_list() {
    let s = String::from("1");
    assert_eq!(parse_list(&s).unwrap(), vec![1]);
    let s = String::from("1,2,3");
    assert_eq!(parse_list(&s).unwrap(), vec![1,2,3]);
    let s = String::from("1-4");
    assert_eq!(parse_list(&s).unwrap(), vec![1,2,3,4]);
}


#[test]
fn test_control_print() {
    let v = vec![1,2,3];
    let a = control_print(&v, &vec![0,3,2], None);
}


#[test]
fn test_answer() {
    let range = "1-2";
    println!("{:?}", range.split(',').into_iter());
    assert_eq!(range.starts_with('+').then(f), true);
    let a = "1,3";
    let b = a.chars().collect::<Vec<_>>();
    let c = b.get(0..2);
    let a = a;
    println!("{:?}", c);
}