use crate::trapdoors::util::integer_to_bits;
use ndarray::linalg::kron;
use ndarray::Array1;
use ndarray::Array2;

/// Gadget basis S_k from trapdoor paper

pub fn generate_full_gadget_basis(q: u32, k: usize, n: usize) -> Array2<i32> {
    let single_gadget_basis = generate_gadget_basis(q, k);
    kron(&Array2::eye(n), &single_gadget_basis)
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
    use crate::trapdoors::gadget_lattice::integer_to_bits;
    use ndarray::array;

    use super::{generate_full_gadget_basis, generate_gadget_basis};

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
