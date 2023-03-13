fn main() {
    if let Ok(c) = catr::get_flags() {
        println!("{:?}", c);
    }
    catr::run(catr::get_flags().unwrap()).expect("fail");
}
