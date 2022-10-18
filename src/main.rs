use short_bases::trapdoors;

fn main() {
    let (a, r) = trapdoors::matrix_with_trapdoor::gen_trap();

    println!("R");
    println!("{}", &r);

    println!("A");
    println!("{}", &a);
}
