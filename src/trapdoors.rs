use ndarray::concatenate;
use ndarray::Array2;
use ndarray::Axis;

mod gadget;
mod sample;

pub fn gen_trap() -> (Array2<u32>, Array2<i32>) {
    let q: u32 = 13;
    // k = ceil(log(q))
    let k: usize = (32 - q.trailing_zeros()).try_into().unwrap();
    // A=[A_bar|A_2] has dimension n x m
    let n = 3;
    // w #columns of of A_2
    let w = n * k;
    // A_bar has dimension n x m_bar
    let m_bar = 3;
    // m = m_bar + w
    let m = m_bar + w;

    assert!(m >= w);
    assert!(w >= n);

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
