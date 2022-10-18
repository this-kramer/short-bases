mod gadget;
mod gadget_lattice;
pub mod matrix_with_short_basis;
mod matrix_with_trapdoor;
mod matrix_with_trapdoor_basis;
mod sample;
mod util;

pub struct GadgetParameters {
    q: u32,
    k: usize,
    n: usize,
    m: usize,
    m_bar: usize,
    w: usize,
}

impl GadgetParameters {
    pub fn new(q: u32, n: usize, m_bar: usize) -> Self {
        let k = util::log_ceil(q).try_into().unwrap();
        let w = n * k;
        let m = m_bar + w;
        assert!(m >= w);
        assert!(w >= n);
        Self {
            q,
            k,
            n,
            m,
            m_bar,
            w,
        }
    }
}
