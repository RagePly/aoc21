#[cfg(feature = "benchmark")]
use std::time::{Instant, Duration};

#[cfg(not(feature = "benchmark"))]
use std::time::Instant;

#[cfg(test)]
mod tests; // nightly needs to be enabled, add #![feature(test)] to the top of this file

pub mod comp_features;
pub mod linalg;
pub mod map;
extern crate num_traits;
#[allow(dead_code, unused_variables)] mod day1;
#[allow(dead_code, unused_variables)] mod day2;
#[allow(dead_code, unused_variables)] mod day3;
#[allow(dead_code, unused_variables)] mod day4;
#[allow(dead_code, unused_variables)] mod day5;
#[allow(dead_code, unused_variables)] mod day6;
#[allow(dead_code, unused_variables)] mod day7;
#[allow(dead_code, unused_variables)] mod day8;
#[allow(dead_code, unused_variables)] mod day9;
#[allow(dead_code, unused_variables)] mod day10;
#[allow(dead_code, unused_variables)] mod day11;
#[allow(dead_code, unused_variables)] mod day12;
#[allow(dead_code, unused_variables)] mod day13;
#[allow(dead_code, unused_variables)] mod day14;
#[allow(dead_code, unused_variables)] mod day15;
#[allow(dead_code, unused_variables)] mod day16;
#[allow(dead_code, unused_variables)] mod day17;
#[allow(dead_code, unused_variables)] mod day18;
#[allow(dead_code, unused_variables)] mod day19;
#[allow(dead_code, unused_variables)] mod day20;
#[allow(dead_code, unused_variables)] mod day21;
#[allow(dead_code, unused_variables)] mod day22;
#[allow(dead_code, unused_variables)] mod day23;
#[allow(dead_code, unused_variables)] mod day24;
#[allow(dead_code, unused_variables)] mod day25;

const FORMAT_PART_WIDTH: usize = 13;
const FORMAT_ANSWER_WIDTH: usize = 64;
const FORMAT_TIME_WIDTH: usize = 9;
const FORMAT_TOTAL_WIDTH: usize = 4 + FORMAT_PART_WIDTH + FORMAT_ANSWER_WIDTH + FORMAT_TIME_WIDTH;
const DATA_FOLDER: &'static str = "./data/";

#[cfg(feature = "benchmark")]
const CYCLE_MAX_LOOP: usize = 100_000;

#[cfg(feature = "benchmark")]
const CYCLE_MAX_TIME: f32 = 5.0;  

#[cfg(feature = "benchmark")]
fn mean(duration: &Vec<Duration>) -> Duration {
    Duration::from_micros(duration.iter().sum::<Duration>().as_micros() as u64 / duration.len() as u64)
}

#[cfg(feature = "benchmark")]
fn std_deviation(durations: &Vec<Duration>) -> u128 {
    let m = mean(durations);
    (durations.iter().map(|dur| (dur.as_micros() as i128 - m.as_micros() as i128).pow(2)).sum::<i128>() as f64 / durations.len() as f64).sqrt() as u128
}

#[cfg(not(feature = "benchmark"))]
macro_rules! execute_task {
    ($part: path, $source: expr) => {
        let pre = Instant::now();
        let res = $part($source);
        let post = pre.elapsed();
        println!("{0:<2$} {1:>3$}µs", 
            res, 
            post.as_micros(),
            FORMAT_ANSWER_WIDTH,
            FORMAT_TIME_WIDTH
        );
    };
}

#[cfg(feature = "benchmark")]
macro_rules! execute_task {
    ($part: path, $source: expr) => {
        let mut i = 0;
        let mut times: Vec<Duration> = Vec::new();
        let mut pre = Instant::now();
        let res = $part($source);
        let mut post = pre.elapsed();
        let mut total = post;
        times.push(post);
        while i < CYCLE_MAX_LOOP && total.as_secs_f32() < CYCLE_MAX_TIME {
            pre = Instant::now();
            let res2 = $part($source);
            post = pre.elapsed();
            total += post;
            times.push(post);
            i += if res2 == res {
                1
            } else {
                panic!()
            };
        }
        println!("{0:<2$} {1:>3$}µs", 
            res, 
            format!("{}±{}", mean(&times).as_micros(), std_deviation(&times)),
            FORMAT_ANSWER_WIDTH,
            FORMAT_TIME_WIDTH
        );
    };
}

macro_rules! execute_day {
    ($day: path) => {
        {
            use $day as base;
            if cfg!(feature = "use_default") || base::WAS_COMPILED {
                let file_path = format!("{}{}.txt", DATA_FOLDER, stringify!($day));
                match std::fs::read_to_string(file_path.clone())  {
                    Ok(source) => {
                        print!("{0:<1$} ", format!("{}:{}:", stringify!($day), "part1"), FORMAT_PART_WIDTH);
                        execute_task!(base::part1, source.clone());
                        print!("{0:<1$} ", format!("{}:{}:", stringify!($day), "part2"), FORMAT_PART_WIDTH);
                        execute_task!(base::part2, source.clone());
                    }
                    Err(_) => println!("{0:<2$} puzzle input (file {1}) not found!", 
                        format!("{}:", stringify!($day)), 
                        file_path, 
                        FORMAT_PART_WIDTH)
                }
            }
        }
    };
}

fn main() {
    println!("{0:^1$}", "Advent of Code 2021", FORMAT_TOTAL_WIDTH);
    println!("{0:<3$} {1:<4$} {2:<5$}", 
        "Puzzle:", "Answer", "Time",
        FORMAT_PART_WIDTH, FORMAT_ANSWER_WIDTH, FORMAT_TIME_WIDTH
    );
    println!("{1:-<0$}", FORMAT_TOTAL_WIDTH, "");
    execute_day!(day1);
    execute_day!(day2);
    execute_day!(day3);
    execute_day!(day4);
    execute_day!(day5);
    execute_day!(day6);
    execute_day!(day7);
    execute_day!(day8);
    execute_day!(day9);
    execute_day!(day10);
    execute_day!(day11);
    execute_day!(day12);
    execute_day!(day13);
    execute_day!(day14);
    execute_day!(day15);
    execute_day!(day16);
    execute_day!(day17);
    execute_day!(day18);
    execute_day!(day19);
    execute_day!(day20);
    execute_day!(day21);
    execute_day!(day22);
    execute_day!(day23);
    execute_day!(day24);
    execute_day!(day25);
}
