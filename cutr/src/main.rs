fn main() {
    match cutr::get_args() {
        Ok(config) => {
            println!("{:?}", config);
            cutr::run(config).unwrap();
        },
        Err(e) => println!("Error: {}", e),
    }
}
