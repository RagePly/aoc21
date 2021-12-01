// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day1");

#[cfg(not(feature = "day1"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(1, 1)
}

#[cfg(not(feature = "day1"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(1, 2)
}

// v START SOLUTION v

#[cfg(feature = "day1")]
pub fn part1(source: String) -> i32 {
    let v: Vec<i32> = source.split("\r\n").map(|line| line.parse().unwrap()).collect();
    v.as_slice().windows(2).map(|v| if v[0] < v[1] { 1 } else { 0 }).sum()
}

#[cfg(feature = "day1")]
pub fn part2(source: String) -> i32 {
    let v: Vec<i32> = source.split("\r\n").map(|line| line.parse().unwrap()).collect();
    let v2: Vec<i32> = v.as_slice().windows(3).map(|v| v.iter().sum()).collect();
    v2.as_slice().windows(2).map(|v| if v[0] < v[1] { 1 } else { 0 }).sum()
}
