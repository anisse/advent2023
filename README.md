My rust workspace for [advent of code 2023](https://adventofcode.com/2023). Every solution could be improved upon, but should at least be readable; this was written mostly as a learning opportunity. I'm putting it out there to expose the process of learning.

I recommend attempting to solve the puzzles before looking at the solutions.

# [What I learned during Advent of Code 2023](https://anisse.astier.eu/aoc-2023-lessons.html).

# Building

The inputs are built-in and you need to have them in `src/inputs` in order to build the project. Otherwise you can use the CI feature and use the samples instead of real inputs:

```sh
# If you put your inputs in src/inputs/
cargo build --release
# Otherwise, just use the samples
cargo build --release --features ci_no_input
```
