use short_bases::{matrix_with_short_basis::generate, GadgetParameters};

fn main() {
    let params = GadgetParameters::new(8389559, 256, 100);
    let (a, s_a) = generate(&params);

    println!("A");
    println!("{}", &a);

    println!("S_A");
    println!("{}", &s_a);
}
