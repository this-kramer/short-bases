use ndarray::Array2;
use rand::Rng;

pub fn sample_a_bar(n: usize, m_bar: usize, q: u32) -> Array2<u32> {
    Array2::from_shape_simple_fn((n, m_bar), || uniform_element_zq(q))
}

pub fn sample_r(m_bar: usize, w: usize) -> Array2<i32> {
    Array2::from_shape_simple_fn((m_bar, w), sample_element)
}

// Sample from distribution with probability 1/2 for 0 and 1/4 each for +/- 1
fn uniform_element_zq(q: u32) -> u32 {
    rand::thread_rng().gen_range(0..q)
}

// Sample from distribution with probability 1/2 for 0 and 1/4 each for +/- 1
fn sample_element() -> i32 {
    let positive: bool = rand::random();
    let is_zero: bool = rand::random();

    if is_zero {
        0
    } else {
        if positive {
            1
        } else {
            -1
        }
    }
}