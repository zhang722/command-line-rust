fn main() {
    let config = uniqr::get_flags().unwrap();
    uniqr::run(&config).unwrap();
}