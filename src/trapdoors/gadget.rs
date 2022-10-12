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

#[cfg(test)]
mod tests {
    use super::{gen_gadget, gen_gadget_matrix};
    use ndarray::array;

    #[test]
    fn gadget() {
        assert_eq!(gen_gadget(4), array![1, 2, 4, 8])
    }

    #[test]
    fn gadget_matrix() {
        assert_eq!(
            gen_gadget_matrix(2, 4),
            array![[1, 2, 4, 8, 0, 0, 0, 0], [0, 0, 0, 0, 1, 2, 4, 8]]
        )
    }
}
