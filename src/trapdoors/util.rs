use bitvec::prelude::*;
use ndarray::Array1;

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

#[cfg(test)]
mod tests {
    use crate::trapdoors::util::integer_to_bits_array_of_size;

    use super::integer_to_bits;
    use ndarray::array;

    #[test]
    fn q_decomposition() {
        assert_eq!(integer_to_bits(6), array![1, 1, 0]);
        assert_eq!(integer_to_bits_array_of_size(6, 3), array![1, 1, 0]);
    }
}
