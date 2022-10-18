use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;

use ndarray::array;
use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;

use crate::gadget_lattice;
use crate::util;
use crate::GadgetParameters;

pub fn short_basis_for_lattice_with_trapdoor(
    a: Array2<u32>,
    r: Array2<i32>,
    params: GadgetParameters,
) -> Array2<i32> {
    let s = gadget_lattice::generate_full_gadget_basis(params.q, params.k, params.n);
    let w = compute_w_matrix(a, params.n, params.k);
    short_basis(r, w, s, params.m, params.w)
}

/// Compute S=[[I, R], [0, I]] * [[I, 0], [W, S]]
fn short_basis(
    r: Array2<i32>,
    w: Array2<i32>,
    s: Array2<i32>,
    m: usize,
    w_dim: usize,
) -> Array2<i32> {
    let rw = r.dot(&w);
    let rs = r.dot(&s);
    let mut s_a: Array2<i32> = Array2::zeros((m, m));

    let mut upper_left = s_a.slice_mut(s![..(m - w_dim), ..(m - w_dim)]);
    let eye: Array2<i32> = Array2::eye(m - w_dim);
    upper_left.assign(&eye.add(rw));

    let mut upper_right = s_a.slice_mut(s![..(m - w_dim), (m - w_dim)..]);
    upper_right.assign(&rs);

    let mut lower_left = s_a.slice_mut(s![(m - w_dim).., ..(m - w_dim)]);
    lower_left.assign(&w);

    let mut lower_right = s_a.slice_mut(s![(m - w_dim).., (m - w_dim)..]);
    lower_right.assign(&s);

    s_a
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
    use super::{compute_w_matrix, short_basis};
    use ndarray::{arr1, array};

    #[test]
    fn correctness() {
        let expected = array![[0, 0], [0, -1], [-1, 0], [-1, -1], [0, 0], [0, -1]];
        let result = compute_w_matrix(array![[1, 2, 3], [4, 5, 6]], 2, 3);
        assert_eq!(expected, result);
    }

    #[test]
    fn short_bases() {
        let m = 5;
        let w_dim = 2;
        let r = array![[1, -2], [0, 1], [-1, 0]];
        let w = array![[1, -2, 1], [-1, 0, 1]];
        let s = array![[1, 2], [-1, -2]];
        let result = short_basis(r, w, s, m, w_dim);
        println!("{}", result);

        assert!(false);
    }
}
