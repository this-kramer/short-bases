use crate::gadget;
use crate::sample;
use crate::GadgetParameters;
use ndarray::concatenate;
use ndarray::Array2;
use ndarray::Axis;

/// Generates A, R where R is a G-trapdoor for A and A is statistically close to uniform.
pub fn gen_trap(params: &GadgetParameters) -> (Array2<u32>, Array2<i32>) {
    let q: u32 = params.q;
    // k = ceil(log(q))
    let k: usize = params.k;
    // A=[A_bar|A_2] has dimension n x m
    let n = params.n;
    // w #columns of of A_2
    let w = params.w;
    // A_bar has dimension n x m_bar
    let m_bar = params.m_bar;

    let g = gadget::gen_gadget_matrix(n, k);
    let g = g.mapv(i64::from);
    let a_bar = sample::sample_a_bar(n, m_bar, q);
    let r = sample::sample_r(m_bar, w);

    // todo multiplication mod q
    let a_bar_i64 = a_bar.mapv(i64::from);
    let r_i64 = r.mapv(i64::from);
    let a_2 = g - a_bar_i64.dot(&r_i64);
    let a_2 = a_2.mapv_into_any(|x| {
        let x = x.rem_euclid(q.try_into().unwrap());
        u32::try_from(x).unwrap()
    });
    let a = concatenate!(Axis(1), a_bar, a_2);

    (a, r)
}
