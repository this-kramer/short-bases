use short_bases;

fn main() {
    let params = short_bases::GadgetParameters::new(8389559, 256, 100);
    let (a, s_a) = short_bases::generate(&params);

    println!("A");
    println!("{}", &a);

    println!("S_A");
    println!("{}", &s_a);
}
