fn main() {
    if let Ok(config) = tailr::get_flags() {
        println!("{:?}", config);
        tailr::run(&config).unwrap();
    }
}
