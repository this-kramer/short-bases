use ndarray::concatenate;
use ndarray::Axis;
use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;
use rand::Rng;

fn main() {
    let r = sample_r(2, 5);
    println!("R");
    println!("{}", &r);

    let a = sample_a_bar(2, 5, 31);
    println!("A");
    println!("{}", &a);

    let g = gen_gadget_matrix(3, 5);
    println!("G");
    println!("{}", &g);
}

fn gen_trap() -> (Array2<u32>, Array2<i32>) {
    let q: u32 = 31;
    let n = 31;
    let w = 4;
    let m_bar = 31;
    let k : usize = (32 - q.trailing_zeros()).try_into().unwrap();
    
    let g = gen_gadget_matrix(n, k);
    let a_bar = sample_a_bar(n, m_bar, q);
    let r = sample_r(m_bar, w);

    // todo multiplication mod q
    let a = concatenate!(Axis(1), a_bar, g - a_bar.dot(&r));

    (a, r)
}

fn gen_gadget_matrix(n: usize, k: usize) -> Array2<u32> {
    let gadget = Array1::from_shape_fn((k), |i| 2u32.pow(i.try_into().unwrap()));
    let mut g = Array2::zeros((n, n * k));
    for i in 0..n {
        let mut slice_for_gadget = g.slice_mut(s![i, k * i..(k * (i + 1))]);
        slice_for_gadget.assign(&gadget);
    }
    g
}

fn sample_a_bar(n: usize, m_bar: usize, q: u32) -> Array2<u32> {
    Array2::from_shape_simple_fn((n, m_bar), || uniform_element_zq(q))
}

fn sample_r(m_bar: usize, w: usize) -> Array2<i32> {
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
