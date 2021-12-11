# Advent Of Code 2021
This is my repository for this year's **AoC**! All solutions will be written in **Rust** and I will be incrementally building a algorithm-library as I go. 
## Running the application
In order to reduce compilation-time while still gathering all solutions in the same crate I'm making use of conditional compilation. Each day has a corresponding *feature*
that control's wether the solution is compiled or not. The feature can be enabled by either editing the `default = [day#]` to the desired day in the `Cargo.toml` file or
specifying the features when building the application: `cargo build --feature day#`. The `all` feature compiles all days.

In order to run test, *nightly* is required.

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
day7:part1:   347509                                                               85±22µs
day7:part2:   98257206                                                             85±23µs
day8:part1:   255                                                                  66±35µs
day8:part2:   982158                                                            2335±373µs
day9:part1:   572                                                               1674±280µs
day9:part2:   847044                                                            3093±441µs
day10:part1:  390993                                                              104±38µs
day10:part2:  2391385187                                                          103±22µs
day11:part1:  1732                                                                524±86µs
day11:part2:  290                                                                799±100µs
```

### Compared to cargo's unittesting
Result of running `cargo bench --features complete`
```
test tests::day1_part1 ... bench:      41,866 ns/iter (+/- 23,032)
test tests::day1_part2 ... bench:      47,561 ns/iter (+/- 18,806)
test tests::day2_part1 ... bench:     344,261 ns/iter (+/- 112,992)
test tests::day2_part2 ... bench:     366,400 ns/iter (+/- 123,315)
test tests::day3_part1 ... bench:      44,202 ns/iter (+/- 17,367)
test tests::day3_part2 ... bench:     188,588 ns/iter (+/- 93,137)
test tests::day4_part1 ... bench:   1,085,531 ns/iter (+/- 216,061)
test tests::day4_part2 ... bench:   1,179,340 ns/iter (+/- 274,931)
test tests::day5_part1 ... bench:   3,579,460 ns/iter (+/- 2,051,551)
test tests::day5_part2 ... bench:   2,357,892 ns/iter (+/- 805,349)
test tests::day6_part1 ... bench:      22,836 ns/iter (+/- 9,295)
test tests::day6_part2 ... bench:     117,184 ns/iter (+/- 46,540)
```