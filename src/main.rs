use short_bases::trapdoors;

fn main() {
    let (a, r) = trapdoors::gen_trap();

    println!("R");
    println!("{}", &r);

    println!("A");
    println!("{}", &a);
}
