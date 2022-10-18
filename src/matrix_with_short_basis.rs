use ndarray::Array2;

use crate::{
    matrix_with_trapdoor, matrix_with_trapdoor_basis::short_basis_for_lattice_with_trapdoor,
    GadgetParameters,
};

pub fn generate(params: &GadgetParameters) -> (Array2<u32>, Array2<i32>) {
    let (a, r) = matrix_with_trapdoor::gen_trap(params);
    let s_a = short_basis_for_lattice_with_trapdoor(&a, &r, params);
    (a, s_a)
}

#[cfg(test)]
mod test {
    use ndarray::{Array1, Array2};

    use crate::GadgetParameters;

    use super::generate;

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
}
