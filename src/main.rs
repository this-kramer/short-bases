use short_bases::{matrix_with_trapdoor, GadgetParameters};
fn main() {
    let params = GadgetParameters::new(17, 3, 4);
    let (a, r) = matrix_with_trapdoor::gen_trap(params);

    println!("R");
    println!("{}", &r);

    println!("A");
    println!("{}", &a);
}
