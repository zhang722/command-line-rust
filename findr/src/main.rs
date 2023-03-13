fn main() {
    let config = findr::get_args().unwrap();
    println!("{:?}", config);
    findr::run(config).unwrap();
    let a = vec!["1", "2", "3"];
    let b = a.join(" ");
    println!("{}", b);
}
 
