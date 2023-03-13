fn main() {
    let config = wcr::get_flags().unwrap();
    wcr::run(&config).unwrap();
    let b = "s";
    let c = &b;
    assert_eq!(c, &"s");
    let mut a: u32 = 1;
    a = a;
    fun(&a);
}

fn fun(mut x: & u32) {
    println!("{}", x);
    x = &3;
    println!("{}", x);
}     
