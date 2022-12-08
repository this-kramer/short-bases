use bitvec::prelude::*;
use ndarray::{Array1, Array2};
use rand::Rng;

/// Returns an array of bools representing the bits of q starting with the first bit that is 1
pub fn integer_to_bits(q: u32) -> Array1<i32> {
    q.view_bits::<Msb0>()
        .into_iter()
        .skip_while(|x| !x.as_ref())
        .map(|x| (*x).into())
        .collect()
}

/// Returns an array of the k least significant bits of u
/// Throws an error if k is larger than 32
pub fn integer_to_bits_array_of_size(u: u32, k: usize) -> Array1<i32> {
    assert!(k <= 32);
    u.view_bits::<Msb0>()
        .into_iter()
        .skip(32 - k)
        .map(|x| (*x).into())
        .collect()
}

/// Compute the logarithm and round up to the next integer
/// Needs to satisfy `x<2^k` for k being the return value
pub fn plus_one_and_log_ceil(x: u32) -> u32 {
    let x = x + 1; // Add 1 to adhere to definition in MP11
    32 - x.checked_sub(1).unwrap().leading_zeros()
}

/// Sample a matrix with entries drawn uniformly at random from ZZ_q
pub fn sample_uniform_matrix(rows: usize, columns: usize, q: u32) -> Array2<u32> {
    Array2::from_shape_simple_fn((rows, columns), || uniform_element_zq(q))
}

/// Sample an element uniformly at random from ZZ_q
fn uniform_element_zq(q: u32) -> u32 {
    rand::thread_rng().gen_range(0..q)
}

/// Samples a matrix with entries in -1,0,+1
pub fn sample_matrix_from_distribution(
    rows: usize,
    columns: usize,
    t: &dyn TrapdoorDistribution,
) -> Array2<i32> {
    Array2::from_shape_simple_fn((rows, columns), || t.sample_element())
}

/// Trait for defining custom trapdoor distributions
pub trait TrapdoorDistribution {
    fn sample_element(&self) -> i32;
}

pub struct PlusMinusOneZero;

impl TrapdoorDistribution for PlusMinusOneZero {
    /// Sample from distribution with probability 1/2 for 0 and 1/4 each for +/- 1
    fn sample_element(&self) -> i32 {
        let positive: bool = rand::random();
        let is_zero: bool = rand::random();

        if is_zero {
            0
        } else if positive {
            1
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{integer_to_bits_array_of_size, plus_one_and_log_ceil};

    use super::integer_to_bits;
    use ndarray::array;

    #[test]
    fn q_decomposition() {
        assert_eq!(integer_to_bits(6), array![1, 1, 0]);
        assert_eq!(integer_to_bits_array_of_size(6, 3), array![1, 1, 0]);
    }

    #[test]
    fn log_powers_of_two() {
        assert_eq!(plus_one_and_log_ceil(1), 1);
        assert_eq!(plus_one_and_log_ceil(2), 2);
        assert_eq!(plus_one_and_log_ceil(4), 3);
        assert_eq!(plus_one_and_log_ceil(8), 4);
        assert_eq!(plus_one_and_log_ceil(16), 5);
    }

    #[test]
    fn log_intermediate_values() {
        assert_eq!(plus_one_and_log_ceil(3), 2);
        assert_eq!(plus_one_and_log_ceil(5), 3);
        assert_eq!(plus_one_and_log_ceil(6), 3);
        assert_eq!(plus_one_and_log_ceil(7), 3);
    }
}
