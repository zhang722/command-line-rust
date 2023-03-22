fn main() {
    if let Ok(config) = calr::get_flags() {
        println!("{:?}", config);
        calr::run(&config).unwrap();
    }
}
