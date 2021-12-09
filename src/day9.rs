// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day9");

#[cfg(not(feature = "day9"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(9, 1)
}

#[cfg(not(feature = "day9"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(9, 2)
}

// v START SOLUTION v

#[cfg(feature = "day9")]
type HeightMap = Vec<Vec<usize>>;

#[cfg(feature = "day9")]
fn is_lower(x1: i32, y1: i32, x2: i32, y2: i32, hm: &HeightMap) -> bool {
    if x2 == -1 || x2 == hm[0].len() as i32 || y2 == -1 || y2 == hm.len() as i32 {
        true
    } else {
        hm[y1 as usize][x1 as usize] < hm[y2 as usize][x2 as usize]
    }
}

#[cfg(feature = "day9")]
fn is_dip(x: i32, y: i32, hm: &HeightMap) -> bool {
    is_lower(x, y, x+1, y, hm) &&
    is_lower(x, y, x-1, y, hm) &&
    is_lower(x, y, x, y+1, hm) &&
    is_lower(x, y, x, y-1, hm)
}

#[cfg(feature = "day9")]
pub fn part1(source: String) -> usize {
    let height_map: HeightMap = source
        .split("\r\n")
        .map(
            |line| line
                .chars()
                .map(
                    |c| String::from(c).as_str().parse::<usize>().unwrap() // fucking hell 
                )
                .collect()
        )
        .collect();
    (0..height_map.len()).map(
        |y| (0..height_map[0].len()).map(
            |x| if is_dip(x as i32, y as i32, &height_map) {
                height_map[y][x] + 1
            } else {
                0
            }
        )
        .sum::<usize>()
    ).sum()
}

#[cfg(feature = "day9")]
use std::collections::{HashSet, VecDeque};


#[cfg(feature = "day9")]
fn is_flowing(x1: i32, y1: i32, x2: i32, y2: i32, hm: &HeightMap) -> bool {
    if 0 <= x2 && x2 < hm[0].len() as i32 && 0 <= y2 && y2 < hm.len() as i32 {
        let other_point = hm[y2 as usize][x2 as usize];
        other_point != 9 && hm[y1 as usize][x1 as usize] < other_point
    } else {
        false
    }
}

#[cfg(feature = "day9")]
fn sizeof_basin(x: usize, y: usize, hm: &HeightMap) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((x, y));
    loop {
        match queue.pop_front() {
            Some(next_point) => {
                if visited.contains(&next_point) {
                    continue
                }

                visited.insert(next_point);
                let x = next_point.0 as i32;
                let y = next_point.1 as i32;
                if is_flowing(x, y, x+1, y, hm) {
                    let new_point = (x as usize +1, y as usize);
                    queue.push_back(new_point);
                }
                if is_flowing(x, y, x-1, y, hm) {
                    let new_point = (x as usize - 1, y as usize);
                    queue.push_back(new_point);
                }
                if is_flowing(x, y, x, y + 1, hm) {
                    let new_point = (x as usize, y as usize + 1);
                    queue.push_back(new_point);
                }
                if is_flowing(x, y, x, y - 1, hm) {
                    let new_point = (x as usize, y as usize - 1);
                    queue.push_back(new_point);
                }
            }
            None => break
        }
    }
    visited.len()
}

#[cfg(feature = "day9")]
pub fn part2(source: String) -> usize {
    let height_map: HeightMap = source
        .split("\r\n")
        .map(
            |line| line
                .chars()
                .map(
                    |c| String::from(c).as_str().parse::<usize>().unwrap() // fucking hell 
                )
                .collect()
        )
        .collect();
    
    let mut basins: Vec<usize> = Vec::new();
    (0..height_map.len()).for_each(
        |y| (0..height_map[0].len()).for_each(
            |x| if height_map[y][x] != 9 && is_dip(x as i32, y as i32, &height_map) {
                basins.push(sizeof_basin(x, y, &height_map))
            }
        )
    );

    basins.sort_unstable();
    basins.as_slice()[basins.len()-3..].iter().product()
}
