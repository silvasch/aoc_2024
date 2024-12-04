# Advent of Code 2024

These are my solutions to advent of code 2024.

## Inputs

The repository does not include the inputs, so you will
have to provide your own. The inputs are stored in `inputs/day{}.txt`, where `{}`
is the day the input zero-padded to two digits (e. g. `05`).

## Running

Calculate the solution for both part one and two for a specific day:

```bash
cargo run --bin day03
```

Calculate the solution for a specific part of a specific day:

```bash
cargo run --bin day03 --no-default-features --features part-one
```

Test teh solution for a specific day using the example input and output:

```bash
cargo test --bin day03
```
