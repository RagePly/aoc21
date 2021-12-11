// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day11");

#[cfg(not(feature = "day11"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(11, 1)
}

#[cfg(not(feature = "day11"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(11, 2)
}

// v START SOLUTION v

#[cfg(feature = "day11")]
use std::collections::VecDeque;

#[cfg(feature = "day11")]
pub fn part1(source: String) -> usize {
    let mut map: Vec<Vec<usize>> = source
    .split("\r\n")
    .map(
        |line| line
            .chars()
            .map(
                |c| (c as u8) as usize - 48 
            )
            .collect()
    )
    .collect();

    let mut blinks: usize = 0;
    for _ in 0..100 {
        let mut queue = VecDeque::<(usize, usize)>::new();

        // find natural blinks + increment all squid
        map
        .iter_mut()
        .enumerate()
        .for_each(
            |(y, row)| row
            .iter_mut()
            .enumerate()
            .for_each(
                |(x, cell)| {
                    *cell += 1;
                    if *cell > 9 {
                        *cell = 0;
                        blinks += 1;
                        queue.push_back((x, y + 1));
                        queue.push_back((x, y.wrapping_sub(1)));
                        queue.push_back((x + 1, y));
                        queue.push_back((x + 1, y + 1));
                        queue.push_back((x + 1, y.wrapping_sub(1)));
                        queue.push_back((x.wrapping_sub(1), y));
                        queue.push_back((x.wrapping_sub(1), y + 1));
                        queue.push_back((x.wrapping_sub(1), y.wrapping_sub(1)));
                    }
                }
            )
        );

        loop {
            let coord = match queue.pop_front() {
                Some(c) => c,
                None => break
            };

            let x = coord.0;
            let y = coord.1;

            if y >= map.len() || x >= map[0].len() {
                continue
            }

            // increase
            if map[y][x] != 0 {
                map[y][x] += 1;
            } else {
                continue
            }

            if map[y][x] > 9 {
                blinks += 1;
                map[y][x] = 0;
                queue.push_back((x, y + 1));
                queue.push_back((x, y.wrapping_sub(1)));
                queue.push_back((x + 1, y));
                queue.push_back((x + 1, y + 1));
                queue.push_back((x + 1, y.wrapping_sub(1)));
                queue.push_back((x.wrapping_sub(1), y));
                queue.push_back((x.wrapping_sub(1), y + 1));
                queue.push_back((x.wrapping_sub(1), y.wrapping_sub(1)));
            }
        }
    }
    blinks
}

#[cfg(feature = "day11")]
pub fn part2(source: String) -> usize {
    let mut map: Vec<Vec<usize>> = source
    .split("\r\n")
    .map(
        |line| line
            .chars()
            .map(
                |c| (c as u8) as usize - 48 
            )
            .collect()
    )
    .collect();

    let mut i: usize = 0;
    loop {
        i += 1;
        let mut queue = VecDeque::<(usize, usize)>::new();
        let mut blinks:usize = 0;

        // find natural blinks + increment all squid
        map
        .iter_mut()
        .enumerate()
        .for_each(
            |(y, row)| row
            .iter_mut()
            .enumerate()
            .for_each(
                |(x, cell)| {
                    *cell += 1;
                    if *cell > 9 {
                        blinks += 1;
                        *cell = 0;
                        queue.push_back((x, y + 1));
                        queue.push_back((x, y.wrapping_sub(1)));
                        queue.push_back((x + 1, y));
                        queue.push_back((x + 1, y + 1));
                        queue.push_back((x + 1, y.wrapping_sub(1)));
                        queue.push_back((x.wrapping_sub(1), y));
                        queue.push_back((x.wrapping_sub(1), y + 1));
                        queue.push_back((x.wrapping_sub(1), y.wrapping_sub(1)));
                    }
                }
            )
        );

        loop {
            let coord = match queue.pop_front() {
                Some(c) => c,
                None => break
            };

            let x = coord.0;
            let y = coord.1;

            if y >= map.len() || x >= map[0].len() {
                continue
            }

            // increase
            if map[y][x] != 0 {
                map[y][x] += 1;
            } else {
                continue
            }

            if map[y][x] > 9 {
                map[y][x] = 0;
                blinks += 1;
                queue.push_back((x, y + 1));
                queue.push_back((x, y.wrapping_sub(1)));
                queue.push_back((x + 1, y));
                queue.push_back((x + 1, y + 1));
                queue.push_back((x + 1, y.wrapping_sub(1)));
                queue.push_back((x.wrapping_sub(1), y));
                queue.push_back((x.wrapping_sub(1), y + 1));
                queue.push_back((x.wrapping_sub(1), y.wrapping_sub(1)));
            }
        }
        if blinks == map.len() * map[0].len() {
            break
        }
    }
    i
}
