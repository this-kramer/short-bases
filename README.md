# Random Lattice with Short Basis Generator

Generate a statistically close to uniform matrix A together with a short basis for the lattice Lambda^perp(A).

This implementation is based on [MP11](https://eprint.iacr.org/2011/501):
 - We generate a matrix A with G-Trapdoor R.
 - Using Lemma 5.3, we compute a short basis for Lambda^perp(A) from R.
  
## License

This project uses the [MIT License](LICENSE.md).

## Disclaimer

The implementation is neither secure nor efficient, its solely for academic purposes!
