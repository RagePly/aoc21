// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day5");

#[cfg(not(feature = "day5"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(5, 1)
}

#[cfg(not(feature = "day5"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(5, 2)
}

// v START SOLUTION v
#[cfg(feature = "day5")]
use super::linalg::Coord;

#[cfg(feature = "day5")]
use std::cmp::{min, max};

#[cfg(feature = "day5")]
type PipeVector = (Coord, Coord, Coord);

#[cfg(feature = "day5")]
pub fn part1(source: String) -> usize {
    // Parse input and area of map
    let mut max_x = 0;
    let mut max_y = 0;
    let pipes: Vec<PipeVector> = source.split("\r\n").map(
        |line| {
            let mut point_pairs = line.split(" -> ").map(
                |point| {
                    let mut points = point.split(",").map(
                        |coordinate| coordinate.parse().unwrap()
                    );
                    Coord { x: points.next().unwrap(), y: points.next().unwrap() }
                }
            );

            let from_point = point_pairs.next().unwrap();
            let to_point = point_pairs.next().unwrap();

            max_x = max(max(max_x, from_point.x), to_point.x);
            max_y = max(max(max_y, from_point.y), to_point.y);

            let direction = Coord {
                x: (to_point.x - from_point.x).signum(),
                y: (to_point.y - from_point.y).signum()
            };
            (from_point, to_point, direction)
        }
    ).collect();

    // initialize map
    let mut map_tally: Vec<Vec<i32>> = Vec::with_capacity(max_y as usize + 1);
    map_tally.resize(max_y as usize + 1, Vec::with_capacity(max_x as usize + 1));
    map_tally.iter_mut().for_each(|v| v.resize(max_x as usize + 1, 0));

    // update map
    pipes.into_iter().for_each(
        |(from_pipe, to_pipe, direction)| if from_pipe.x == to_pipe.x {
            for y in min(from_pipe.y, to_pipe.y)..max(from_pipe.y, to_pipe.y) + 1 {
                map_tally[y as usize][from_pipe.x as usize] += 1;
            }
        } else if  from_pipe.y == to_pipe.y { 
            for x in min(from_pipe.x, to_pipe.x)..max(from_pipe.x, to_pipe.x) + 1 {
                map_tally[from_pipe.y as usize][x as usize] += 1;
            }
        }
    );

    // tally map
    map_tally.into_iter().map(
        |row| row.iter().filter(
            |point| **point > 1
        ).count()
    ).sum()
}

#[cfg(feature = "day5")]
pub fn part2(source: String) -> usize {
    // parse input and area of map
    let mut max_x = 0;
    let mut max_y = 0;
    let pipes: Vec<PipeVector> = source.split("\r\n").map(
        |line| {
            let mut point_pairs = line.split(" -> ").map(
                |point| {
                    let mut points = point.split(",").map(
                        |coordinate| coordinate.parse().unwrap()
                    );
                    Coord { x: points.next().unwrap(), y: points.next().unwrap() }
                }
            );

            let from_point = point_pairs.next().unwrap();
            let to_point = point_pairs.next().unwrap();

            max_x = max(max(max_x, from_point.x), to_point.x);
            max_y = max(max(max_y, from_point.y), to_point.y);

            let direction = Coord {
                x: (to_point.x - from_point.x).signum(),
                y: (to_point.y - from_point.y).signum()
            };
            (from_point, to_point, direction)
        }
    ).collect();

    // initialize map
    let mut map_tally: Vec<Vec<i32>> = Vec::with_capacity(max_y as usize + 1);
    map_tally.resize(max_y as usize + 1, Vec::with_capacity(max_x as usize + 1));
    map_tally.iter_mut().for_each(|v| v.resize(max_x as usize + 1, 0));

    // update map
    pipes.into_iter().for_each(
        |(from_pipe, to_pipe, direction)| {
            let mut point = from_pipe.clone();
            let destination = to_pipe + direction.clone();
            loop {
                map_tally[point.y as usize][point.x as usize] += 1;
                point += direction.clone();
                if point == destination{
                    break
                }
            }
        }
    );

    // tally map
    map_tally.into_iter().map(
        |row| row.iter().filter(
            |point| **point > 1
        ).count()
    ).sum()
}
