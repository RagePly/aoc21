extern crate test;
use test::Bencher;

#[cfg(feature = "day1")]
#[bench]
fn day1_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day1.txt").unwrap();

    bench.iter(|| super::day1::part1(source.clone()));
}

#[cfg(feature = "day1")]
#[bench]
fn day1_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day1.txt").unwrap();

    bench.iter(|| super::day1::part2(source.clone()));
}

#[cfg(feature = "day2")]
#[bench]
fn day2_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day2.txt").unwrap();

    bench.iter(|| super::day2::part1(source.clone()));
}

#[cfg(feature = "day2")]
#[bench]
fn day2_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day2.txt").unwrap();

    bench.iter(|| super::day2::part2(source.clone()));
}

#[cfg(feature = "day3")]
#[bench]
fn day3_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day3.txt").unwrap();

    bench.iter(|| super::day3::part1(source.clone()));
}

#[cfg(feature = "day3")]
#[bench]
fn day3_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day3.txt").unwrap();

    bench.iter(|| super::day3::part2(source.clone()));
}

#[cfg(feature = "day4")]
#[bench]
fn day4_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day4.txt").unwrap();

    bench.iter(|| super::day4::part1(source.clone()));
}

#[cfg(feature = "day4")]
#[bench]
fn day4_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day4.txt").unwrap();

    bench.iter(|| super::day4::part2(source.clone()));
}

#[cfg(feature = "day5")]
#[bench]
fn day5_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day5.txt").unwrap();

    bench.iter(|| super::day5::part1(source.clone()));
}

#[cfg(feature = "day5")]
#[bench]
fn day5_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day5.txt").unwrap();

    bench.iter(|| super::day5::part2(source.clone()));
}

#[cfg(feature = "day6")]
#[bench]
fn day6_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day6.txt").unwrap();

    bench.iter(|| super::day6::part1(source.clone()));
}

#[cfg(feature = "day6")]
#[bench]
fn day6_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day6.txt").unwrap();

    bench.iter(|| super::day6::part2(source.clone()));
}

#[cfg(feature = "day7")]
#[bench]
fn day7_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day7.txt").unwrap();

    bench.iter(|| super::day7::part1(source.clone()));
}

#[cfg(feature = "day7")]
#[bench]
fn day7_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day7.txt").unwrap();

    bench.iter(|| super::day7::part2(source.clone()));
}

#[cfg(feature = "day8")]
#[bench]
fn day8_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day8.txt").unwrap();

    bench.iter(|| super::day8::part1(source.clone()));
}

#[cfg(feature = "day8")]
#[bench]
fn day8_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day8.txt").unwrap();

    bench.iter(|| super::day8::part2(source.clone()));
}

#[cfg(feature = "day9")]
#[bench]
fn day9_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day9.txt").unwrap();

    bench.iter(|| super::day9::part1(source.clone()));
}

#[cfg(feature = "day9")]
#[bench]
fn day9_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day9.txt").unwrap();

    bench.iter(|| super::day9::part2(source.clone()));
}

#[cfg(feature = "day10")]
#[bench]
fn day10_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day10.txt").unwrap();

    bench.iter(|| super::day10::part1(source.clone()));
}

#[cfg(feature = "day10")]
#[bench]
fn day10_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day10.txt").unwrap();

    bench.iter(|| super::day10::part2(source.clone()));
}

#[cfg(feature = "day11")]
#[bench]
fn day11_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day11.txt").unwrap();

    bench.iter(|| super::day11::part1(source.clone()));
}

#[cfg(feature = "day11")]
#[bench]
fn day11_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day11.txt").unwrap();

    bench.iter(|| super::day11::part2(source.clone()));
}

#[cfg(feature = "day12")]
#[bench]
fn day12_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day12.txt").unwrap();

    bench.iter(|| super::day12::part1(source.clone()));
}

#[cfg(feature = "day12")]
#[bench]
fn day12_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day12.txt").unwrap();

    bench.iter(|| super::day12::part2(source.clone()));
}

#[cfg(feature = "day13")]
#[bench]
fn day13_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day13.txt").unwrap();

    bench.iter(|| super::day13::part1(source.clone()));
}

#[cfg(feature = "day13")]
#[bench]
fn day13_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day13.txt").unwrap();

    bench.iter(|| super::day13::part2(source.clone()));
}

#[cfg(feature = "day14")]
#[bench]
fn day14_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day14.txt").unwrap();

    bench.iter(|| super::day14::part1(source.clone()));
}

#[cfg(feature = "day14")]
#[bench]
fn day14_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day14.txt").unwrap();

    bench.iter(|| super::day14::part2(source.clone()));
}

#[cfg(feature = "day15")]
#[bench]
fn day15_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day15.txt").unwrap();

    bench.iter(|| super::day15::part1(source.clone()));
}

#[cfg(feature = "day15")]
#[bench]
fn day15_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day15.txt").unwrap();

    bench.iter(|| super::day15::part2(source.clone()));
}

#[cfg(feature = "day16")]
#[bench]
fn day16_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day16.txt").unwrap();

    bench.iter(|| super::day16::part1(source.clone()));
}

#[cfg(feature = "day16")]
#[bench]
fn day16_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day16.txt").unwrap();

    bench.iter(|| super::day16::part2(source.clone()));
}

#[cfg(feature = "day17")]
#[bench]
fn day17_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day17.txt").unwrap();

    bench.iter(|| super::day17::part1(source.clone()));
}

#[cfg(feature = "day17")]
#[bench]
fn day17_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day17.txt").unwrap();

    bench.iter(|| super::day17::part2(source.clone()));
}

#[cfg(feature = "day18")]
#[bench]
fn day18_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day18.txt").unwrap();

    bench.iter(|| super::day18::part1(source.clone()));
}

#[cfg(feature = "day18")]
#[bench]
fn day18_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day18.txt").unwrap();

    bench.iter(|| super::day18::part2(source.clone()));
}

#[cfg(feature = "day19")]
#[bench]
fn day19_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day19.txt").unwrap();

    bench.iter(|| super::day19::part1(source.clone()));
}

#[cfg(feature = "day19")]
#[bench]
fn day19_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day19.txt").unwrap();

    bench.iter(|| super::day19::part2(source.clone()));
}

#[cfg(feature = "day20")]
#[bench]
fn day20_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day20.txt").unwrap();

    bench.iter(|| super::day20::part1(source.clone()));
}

#[cfg(feature = "day20")]
#[bench]
fn day20_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day20.txt").unwrap();

    bench.iter(|| super::day20::part2(source.clone()));
}

#[cfg(feature = "day21")]
#[bench]
fn day21_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day21.txt").unwrap();

    bench.iter(|| super::day21::part1(source.clone()));
}

#[cfg(feature = "day21")]
#[bench]
fn day21_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day21.txt").unwrap();

    bench.iter(|| super::day21::part2(source.clone()));
}

#[cfg(feature = "day22")]
#[bench]
fn day22_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day22.txt").unwrap();

    bench.iter(|| super::day22::part1(source.clone()));
}

#[cfg(feature = "day22")]
#[bench]
fn day22_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day22.txt").unwrap();

    bench.iter(|| super::day22::part2(source.clone()));
}

#[cfg(feature = "day23")]
#[bench]
fn day23_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day23.txt").unwrap();

    bench.iter(|| super::day23::part1(source.clone()));
}

#[cfg(feature = "day23")]
#[bench]
fn day23_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day23.txt").unwrap();

    bench.iter(|| super::day23::part2(source.clone()));
}

#[cfg(feature = "day24")]
#[bench]
fn day24_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day24.txt").unwrap();

    bench.iter(|| super::day24::part1(source.clone()));
}

#[cfg(feature = "day24")]
#[bench]
fn day24_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day24.txt").unwrap();

    bench.iter(|| super::day24::part2(source.clone()));
}

#[cfg(feature = "day25")]
#[bench]
fn day25_part1(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day25.txt").unwrap();

    bench.iter(|| super::day25::part1(source.clone()));
}

#[cfg(feature = "day25")]
#[bench]
fn day25_part2(bench: &mut Bencher) {
    let source = std::fs::read_to_string("./data/day25.txt").unwrap();

    bench.iter(|| super::day25::part2(source.clone()));
}