use clap::{ App, Arg };

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("z")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
            .short("n")
            .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = if matches.is_present("omit_newline") { "" } else { "\n" };

    print!("{}{}", text.join(" "), omit_newline);
}
