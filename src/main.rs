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

    let (a, r) = gen_trap();

    println!("R");
    println!("{}", &r);

    println!("A");
    println!("{}", &a);
}

fn gen_trap() -> (Array2<u32>, Array2<i32>) {
    let q: u32 = 13;
    // k = ceil(log(q))
    let k : usize = (32 - q.trailing_zeros()).try_into().unwrap();
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
    
    let g = gen_gadget_matrix(n, k);
    let g = g.mapv(|x| i64::from(x));
    let a_bar = sample_a_bar(n, m_bar, q);
    let r = sample_r(m_bar, w);

    // todo multiplication mod q
    let a_bar_i64 =a_bar.mapv(|x| i64::from(x)); 
    let r_i64 = r.mapv(|x| i64::from(x));
    let a_2 = g - a_bar_i64.dot(&r_i64);
    let a_2 = a_2.mapv_into_any(|x| {
        let x = x.rem_euclid(q.try_into().unwrap());
        u32::try_from(x).unwrap()
       });
    let a = concatenate!(Axis(1), a_bar, a_2);

    (a, r)
}

fn gen_gadget_matrix(n: usize, k: usize) -> Array2<u32> {
    let gadget = Array1::from_shape_fn(k, |i| 2u32.pow(i.try_into().unwrap()));
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
