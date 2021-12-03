# Advent Of Code 2021
This is my repository for this year's **AoC**! All solutions will be written in **Rust** and I will be incrementally building a algorithm-library as I go. 
## Running the application
In order to reduce compilation-time while still gathering all solutions in the same crate I'm making use of conditional compilation. Each day has a corresponding *feature*
that control's wether the solution is compiled or not. The feature can be enabled by either editing the `default = [day#]` to the desired day in the `Cargo.toml` file or
specifying the features when building the application: `cargo build --feature day#`. The `all` feature compiles all days.

## Benchmark
Result of running `cargo run --release --features complete --features benchmark`
```
                                   Advent of Code 2021
Puzzle:       Answer                                                           Time     
------------------------------------------------------------------------------------------
day1:part1:   1466                                                                 43±11µs
day1:part2:   1491                                                                 44±15µs
day2:part1:   1507611                                                             365±53µs
day2:part2:   1880593125                                                          355±55µs
day3:part1:   3923414                                                            550±130µs
day3:part2:   5852595                                                             172±35µs
```
