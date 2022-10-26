use crate::util::integer_to_bits;
use ndarray::linalg::kron;
use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;

pub fn compute_gadget_matrix(n: usize, k: usize) -> Array2<u32> {
    let gadget = gen_gadget(k);
    let mut g = Array2::zeros((n, n * k));
    for i in 0..n {
        let mut slice_for_gadget = g.slice_mut(s![i, k * i..(k * (i + 1))]);
        slice_for_gadget.assign(&gadget);
    }
    g
}

/// Gadget basis S_k from trapdoor paper
pub fn generate_full_gadget_basis(q: u32, k: usize, n: usize) -> Array2<i32> {
    let single_gadget_basis = generate_gadget_basis(q, k);
    kron(&Array2::eye(n), &single_gadget_basis)
}

fn gen_gadget(k: usize) -> Array1<u32> {
    Array1::from_shape_fn(k, |i| 2u32.pow(i.try_into().unwrap()))
}

fn generate_gadget_basis(q: u32, k: usize) -> Array2<i32> {
    let mut basis = Array2::zeros((k, k));
    for (i, mut column) in basis.columns_mut().into_iter().take(k - 1).enumerate() {
        column[i] = 2;
        column[i + 1] = -1;
    }

    let mut last_column = integer_to_bits(q);
    for i in 0..(k / 2) {
        let tmp = last_column[i];
        last_column[i] = last_column[k - i - 1];
        last_column[k - i - 1] = tmp;
    }

    basis.column_mut(k - 1).assign(&last_column);

    basis
}

#[cfg(test)]
mod tests {
    use super::{compute_gadget_matrix, gen_gadget, integer_to_bits};
    use crate::gadget::{generate_full_gadget_basis, generate_gadget_basis};
    use ndarray::array;

    #[test]
    fn gadget() {
        assert_eq!(gen_gadget(4), array![1, 2, 4, 8])
    }

    #[test]
    fn gadget_matrix() {
        assert_eq!(
            compute_gadget_matrix(2, 4),
            array![[1, 2, 4, 8, 0, 0, 0, 0], [0, 0, 0, 0, 1, 2, 4, 8]]
        )
    }

    #[test]
    fn gadget_matrix_larger() {
        assert_eq!(
            compute_gadget_matrix(3, 2),
            array![[1, 2, 0, 0, 0, 0], [0, 0, 1, 2, 0, 0], [0, 0, 0, 0, 1, 2]]
        )
    }

    #[test]
    fn single_gadget_basis() {
        assert_eq!(
            generate_gadget_basis(4, 3),
            array![[2, 0, 0], [-1, 2, 0], [0, -1, 1]]
        )
    }

    #[test]
    fn full_gadget_basis() {
        assert_eq!(
            generate_full_gadget_basis(4, 3, 2),
            array![
                [2, 0, 0, 0, 0, 0],
                [-1, 2, 0, 0, 0, 0],
                [0, -1, 1, 0, 0, 0],
                [0, 0, 0, 2, 0, 0],
                [0, 0, 0, -1, 2, 0],
                [0, 0, 0, 0, -1, 1]
            ]
        )
    }

    #[test]
    fn q_decomposition() {
        assert_eq!(integer_to_bits(6), array![1, 1, 0])
    }
}
