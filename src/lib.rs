mod gadget;
mod generation;
mod util;

pub use crate::util::PlusMinusOneZero;
pub use crate::util::TrapdoorDistribution;
pub use generation::generate;

/// All the parameters as named in the paper + the distribution to choose the entries of R from.
pub struct GadgetParameters {
    q: u32,
    k: usize,
    n: usize,
    m: usize,
    m_bar: usize,
    w: usize,
    trapdoor_distribution: Box<dyn TrapdoorDistribution>,
}

impl GadgetParameters {
    /// Create gadget parameters with a custom distribution for the trapdoor.
    pub fn new_with_trapdoor_distribution(
        q: u32,
        n: usize,
        m_bar: usize,
        trapdoor_distribution: Box<dyn TrapdoorDistribution>,
    ) -> Self {
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
            trapdoor_distribution,
        }
    }

    /// Create gadget parameters with the -1,0,1 distribution
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
            trapdoor_distribution: Box::new(PlusMinusOneZero),
        }
    }
}
