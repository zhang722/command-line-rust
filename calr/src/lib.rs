use std::{
    error::Error, 
};

use clap::{App, Arg};
use chrono::{Datelike, Local, NaiveDate, Weekday};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub month: usize,
    pub year: usize,
    // month: String,
    // year: String,
    pub show_whole_year: bool,
}

pub fn get_flags() -> MyResult<Config> {
    let matches = App::new("clar")
        .version("0.1.0")
        .author("z") 
        .arg(
            Arg::with_name("month")
                ,
        )
        .arg(
            Arg::with_name("year"),
        )
        .arg(
            Arg::with_name("show-whole-year")
                .short("y")
                .long("year"),
        )
        .get_matches();
    let is_month_present = matches.is_present("month");
    let is_year_present = matches.is_present("year");
    let (month, year) = if is_month_present && is_year_present {
        let month = matches.value_of("month").map(parse::<12>).transpose().unwrap();
        let year = matches.value_of("year").map(parse::<9999>).transpose().unwrap();
        (month.unwrap(), year.unwrap())
    } else if is_month_present && !is_year_present {
        // all monthes of the year
        let year = matches.value_of("month").map(parse::<9999>).transpose().unwrap();
        (0, year.unwrap())
    } else {
        // current month and year
        (0, 0)
    };

    let show = matches.is_present("show-whole-year");
    Ok(Config { month, year, show_whole_year: show })
}

fn parse<const N: usize>(input: &str) -> MyResult<usize> {
    match input.parse::<usize>() {
        Ok(num) => {
            if num > N || num < 1 {
                return Err(From::from(format!("parse failed: {} is not in correct range", num)));
            } else {
                Ok(num)
            }
        },
        Err(e) => {
            Err(From::from(format!("parse failed: {}", e)))
        }
    }
}

pub fn print_month(year: i32, month: u32, day: u32) {
    let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let now = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let days = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .signed_duration_since(start).num_days();
    
    println!("{:^21}", start.format("%B"));
    let a = start.weekday().num_days_from_monday();
    for _ in 0..a {
        print!("   ");
    }
    for each_day in start.iter_days().take(days as usize) {
        if each_day.weekday() == Weekday::Mon {
            println!();
        }
        if each_day == now {
            print!("\x1b[34m{:<3}\x1b[0m", each_day.day());
        } else {
            print!("{:<3}", each_day.day());
        }
    }
    println!();
}

pub fn print_month_nd(nd: NaiveDate) {
    let year = nd.year();
    let month = nd.month();
    let day = nd.day();
    print_month(year, month, day);
}

pub fn run(config: &Config) -> MyResult<()> {
    if config.year == 0 {
        let year = Local::now().date_naive().year();
        println!("{:^21}", year);
    } else {
        println!("{:^21}", config.year);
    }
    match (config.month, config.year) {
        (0, 0) => {
            let today = Local::now().date_naive();
            print_month_nd(today);
        },
        (0, year) => {
            for month in 1..=12 {
                print_month(year as i32, month, 1);
            }
        },
        _ => {
            print_month(config.year as i32, config.month as u32, 1);
        }
    };

    Ok(())
}


#[cfg(test)]
mod test {
    use chrono::{NaiveDate, Weekday, Datelike, Month};

    #[test]
    fn test_show_datelike() -> Result<(), ()> {
        let now = NaiveDate::from_ymd_opt(2023, 2, 1).unwrap();
        let days = NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()
            .signed_duration_since(now).num_days();
        
        println!("{:^21}", now.year());
        println!("{:^21}", now.format("%B"));
        let a = now.weekday().num_days_from_monday();
        for _ in 0..a {
            print!("   ");
        }
        for day in now.iter_days().take(days as usize) {
            if day.weekday() == Weekday::Mon {
                println!();
            }
            if day == now {
                print!("\x1b[34m{:<3}\x1b[0m", day.day());
            } else {
                print!("{:<3}", day.day());
            }
        }
        Err(())
    }
}