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
day1:part1:   1466                                                                  40±8µs
day1:part2:   1491                                                                 45±46µs
day2:part1:   1507611                                                             364±55µs
day2:part2:   1880593125                                                          381±75µs
day3:part1:   3923414                                                              46±14µs
day3:part2:   5852595                                                             178±34µs
day4:part1:   25410                                                             1220±110µs
day4:part2:   2730                                                              1322±135µs
day5:part1:   4421                                                              3621±852µs
day5:part2:   18674                                                             3378±735µs
day6:part1:   374994                                                                25±6µs
day6:part2:   1686252324092                                                       127±23µs
```

### Comment on the performance
Day 6 seems **WAY** to fast for a laptop computer to handle. Mine has a 1.6 GHz processor and with the current performance it would imply that around only 40000 instructions
were used to solve that part. I've modified my benchmark code to do some dummy calculations in order to hopefully keep the compiler from just optimizing my code away. 

As for the duration this text stands, the above benchmark should be considered invalid.