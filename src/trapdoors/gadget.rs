use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;

pub fn gen_gadget_matrix(n: usize, k: usize) -> Array2<u32> {
    let gadget = gen_gadget(k);
    let mut g = Array2::zeros((n, n * k));
    for i in 0..n {
        let mut slice_for_gadget = g.slice_mut(s![i, k * i..(k * (i + 1))]);
        slice_for_gadget.assign(&gadget);
    }
    g
}

fn gen_gadget(k: usize) -> Array1<u32> {
    Array1::from_shape_fn(k, |i| 2u32.pow(i.try_into().unwrap()))
}

