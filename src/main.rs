use crate::example_distribution::UniformOverInterval;

fn main() {
    let params = short_bases::GadgetParameters::new(8389559, 32, 64);
    let (a, s_a) = short_bases::generate(&params);

    println!("A");
    println!("{}", &a);

    println!("S_A");
    println!("{}", &s_a);

    // With custom error distribution
    let custom_distribution = UniformOverInterval::new(-4, 4);
    let params = short_bases::GadgetParameters::new_with_trapdoor_distribution(
        8389559,
        32,
        64,
        Box::new(custom_distribution),
    );
    let (a, s_a) = short_bases::generate(&params);

    println!("A");
    println!("{}", &a);

    println!("S_A");
    println!("{}", &s_a);
}

mod example_distribution {
    use rand::Rng;
    use short_bases::TrapdoorDistribution;

    pub struct UniformOverInterval {
        left: i32,
        right: i32,
    }

    impl UniformOverInterval {
        pub fn new(left: i32, right: i32) -> Self {
            assert!(left <= right);
            UniformOverInterval { left, right }
        }
    }

    impl TrapdoorDistribution for UniformOverInterval {
        fn sample_element(&self) -> i32 {
            rand::thread_rng().gen_range(self.left..=self.right)
        }
    }
}
