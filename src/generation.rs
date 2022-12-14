use crate::{
    gadget,
    util::{self},
    GadgetParameters,
};
use ndarray::Axis;
use ndarray::{concatenate, Array2};
use std::ops::Add;
use std::ops::Neg;

use ndarray::s;
use ndarray::Array1;

/// Generate A together with a short basis s_a for the lattice Lambda^perp(A)
pub fn generate(params: &GadgetParameters) -> (Array2<u32>, Array2<i32>) {
    let (a, r) = gen_trap(params);
    let s_a = short_basis_for_lattice_with_trapdoor(&a, &r, params);
    (a, s_a)
}

/// Generates A, R where R is a G-trapdoor for A and A is statistically close to uniform.
fn gen_trap(params: &GadgetParameters) -> (Array2<u32>, Array2<i32>) {
    let q: u32 = params.q;
    // k = ceil(log(q))
    let k: usize = params.k;
    // A=[A_bar|A_2] has dimension n x m
    let n = params.n;
    // w #columns of of A_2
    let w = params.w;
    // A_bar has dimension n x m_bar
    let m_bar = params.m_bar;

    let g = gadget::compute_gadget_matrix(n, k);
    let g = g.mapv(i64::from);
    let a_bar = util::sample_uniform_matrix(n, m_bar, q);
    let r = util::sample_matrix_from_distribution(m_bar, w, params.trapdoor_distribution.as_ref());

    let a_bar_i64 = a_bar.mapv(i64::from);
    let r_i64 = r.mapv(i64::from);

    // compute a_2 ver the integers and then reduce it modulo q
    let a_2 = g - a_bar_i64.dot(&r_i64);
    let a_2 = a_2.mapv_into_any(|x| {
        let x = x.rem_euclid(q.try_into().unwrap());
        u32::try_from(x).unwrap()
    });
    let a = concatenate!(Axis(1), a_bar, a_2);

    (a, r)
}

fn short_basis_for_lattice_with_trapdoor(
    a: &Array2<u32>,
    r: &Array2<i32>,
    params: &GadgetParameters,
) -> Array2<i32> {
    let s = gadget::generate_full_gadget_basis(params.q, params.k, params.n);
    let w = compute_w_matrix(a, params.n, params.k, params.m_bar);
    short_basis(r, &w, &s, params.m, params.w)
}

/// Compute S=[[I, R], [0, I]] * [[I, 0], [W, S]]
fn short_basis(
    r: &Array2<i32>,
    w: &Array2<i32>,
    s: &Array2<i32>,
    m: usize,
    w_dim: usize,
) -> Array2<i32> {
    let rw = r.dot(w);
    let rs = r.dot(s);
    let mut s_a: Array2<i32> = Array2::zeros((m, m));

    let mut upper_left = s_a.slice_mut(s![..(m - w_dim), ..(m - w_dim)]);
    let eye: Array2<i32> = Array2::eye(m - w_dim);
    upper_left.assign(&eye.add(rw));

    let mut upper_right = s_a.slice_mut(s![..(m - w_dim), (m - w_dim)..]);
    upper_right.assign(&rs);

    let mut lower_left = s_a.slice_mut(s![(m - w_dim).., ..(m - w_dim)]);
    lower_left.assign(w);

    let mut lower_right = s_a.slice_mut(s![(m - w_dim).., (m - w_dim)..]);
    lower_right.assign(s);

    s_a
}

fn compute_w_matrix(a: &Array2<u32>, n: usize, k: usize, m_bar: usize) -> Array2<i32> {
    // First sqaure matrix of a
    let a = a.slice(s![.., ..m_bar]);
    let mut w: Array2<i32> = Array2::zeros((k * n, m_bar));
    for (i, row) in a.rows().into_iter().enumerate() {
        for (j, a_ij) in row.iter().enumerate() {
            let a_ij_binary: Array1<i32> = util::integer_to_bits_array_of_size(*a_ij, k);
            w.slice_mut(s![(k * i)..(k * (i + 1));-1, j])
                .assign(&a_ij_binary);
        }
    }
    w.neg()
}

#[cfg(test)]
mod test {
    use super::compute_w_matrix;
    use super::generate;
    use crate::GadgetParameters;
    use ndarray::array;
    use ndarray::{Array1, Array2};
    use std::ops::Neg;

    #[test]
    fn test_basis_vectors_in_lattice() {
        let params = GadgetParameters::new(17, 5, 8);

        let (a, s_a) = generate(&params);

        let a: Array2<i64> = a.mapv(|x| x as i64);
        for column in s_a.columns() {
            assert_eq!(
                a.dot(&column.map(|x| *x as i64))
                    .map(|x| x.rem_euclid(params.q.into())),
                Array1::zeros(params.n)
            );
        }
    }

    #[test]
    fn correctness() {
        let expected = array![
            [1, 0, 1],
            [0, 1, 1],
            [0, 0, 0],
            [1, 0, 1],
            [0, 1, 1],
            [1, 1, 1]
        ]
        .neg();
        let result = compute_w_matrix(&array![[1, 2, 3, 4], [5, 6, 7, 0]], 2, 3, 3);
        assert_eq!(expected, result);
    }
}
