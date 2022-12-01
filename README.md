# aoc-2022
Advent of code 2022 solutions in Rust.

### Running solutions
A nightly version of rust is required. Solutions can be run with:
```
cargo run --release --bin day<X>
```
where `X` is the day number.

- If you want to download inputs automatically you have to put your session cookie of AoC in a file named `session`. It should be in the format `session=<hex>`.
- Alternatively, you can create the files manually in the `inputs` folder, inputs for day `x` are read from `x.txt`.

To benchmark a solution, pass the `--bench` flag and optimize the binary for your specific CPU like this:
```
RUSTFLAGS="-C target-cpu=native" cargo run --release --bin day<X> -- --bench
```
