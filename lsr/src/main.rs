fn main() {
    if let Ok(config) = lsr::get_flags() {
        println!("{:?}", config);
    }
}
