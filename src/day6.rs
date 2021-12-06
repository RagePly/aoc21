// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day6");

#[cfg(not(feature = "day6"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(6, 1)
}

#[cfg(not(feature = "day6"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(6, 2)
}

// v START SOLUTION v

#[cfg(feature = "day6")]
use std::collections::HashMap;

#[cfg(feature = "day6")]
type ChildCache = HashMap<i64, i64>;

#[cfg(feature = "day6")]
fn nr_children(time_left: i64, child_cache: &mut ChildCache) -> i64 {
    if time_left < 0 {
        0
    } else {
        match child_cache.get(&time_left) {
            Some(c) => *c,
            None => {
                let mut child_sum = 0;
                let mut time_offset = 0;
                while time_left - time_offset > 0{
                    child_sum += 1 + nr_children(time_left - time_offset - 9, child_cache);
                    time_offset += 7;
                }
                child_cache.insert(time_left, child_sum);
                child_sum
            }
        }
    }
}

#[cfg(feature = "day6")]
pub fn part1(source: String) -> i64 {
    // parse input
    let mut child_cache = ChildCache::new();
    let days: i64 = 80;
    source.split(",").map(
        |init_offset| nr_children(days - init_offset.parse::<i64>().unwrap(), &mut child_cache) + 1
    ).sum()
}

#[cfg(feature = "day6")]
pub fn part2(source: String) -> i64 {
    let mut child_cache = ChildCache::new();
    let days: i64 = 256;
    source.split(",").map(
        |init_offset| nr_children(days - init_offset.parse::<i64>().unwrap(), &mut child_cache) + 1
    ).sum()
}
