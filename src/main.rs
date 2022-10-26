fn main() {
    let params = short_bases::GadgetParameters::new(
        8389559,
        32,
        64,
        Box::new(short_bases::PlusMinusOneZero),
    );
    let (a, s_a) = short_bases::generate(&params);

    println!("A");
    println!("{}", &a);

    println!("S_A");
    println!("{}", &s_a);
}
