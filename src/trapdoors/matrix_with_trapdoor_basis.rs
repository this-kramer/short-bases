use std::ops::Neg;

use ndarray::array;
use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;

use crate::trapdoors::util;

fn short_basis(a: Array2<u32>, r: Array2<i32>) -> Array2<i32> {
    array![[1]]
}

/// A m*n
/// R (m-w)*w
/// G w*m_bar
fn compute_w_matrix(a: Array2<u32>, n: usize, k: usize) -> Array2<i32> {
    // First sqaure matrix of a
    let a = a.slice(s![.., 0..n]);
    let mut w: Array2<i32> = Array2::zeros((k * n, n));
    for (i, row) in a.rows().into_iter().enumerate() {
        for (j, a_ij) in row.iter().enumerate() {
            let a_ij_binary: Array1<i32> = util::integer_to_bits_array_of_size(*a_ij, k);
            w.slice_mut(s![(k * i)..(k * (i + 1)), j])
                .assign(&a_ij_binary);
        }
    }
    w.neg()
}

#[cfg(test)]
mod test {
    use super::compute_w_matrix;
    use ndarray::array;

    #[test]
    fn correctness() {
        let expected = array![[0, 0], [0, -1], [-1, 0], [-1, -1], [0, 0], [0, -1]];
        let result = compute_w_matrix(array![[1, 2, 3], [4, 5, 6]], 2, 3);
        assert_eq!(expected, result);
    }
}
