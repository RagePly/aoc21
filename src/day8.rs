// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day8");

#[cfg(not(feature = "day8"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(8, 1)
}

#[cfg(not(feature = "day8"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(8, 2)
}

// v START SOLUTION v

#[cfg(feature = "day8")]
pub fn part1(source: String) -> usize {
    source.split("\r\n").map(
        |line| line
            .splitn(2, " | ")
            .skip(1)
            .next()
            .unwrap()
            .split(" ")
            .filter(
                |word| match word.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false
                }
            ).count()
    ).sum()
}

#[cfg(feature = "day8")]
use std::collections::HashSet;

#[cfg(feature = "day8")]
type Segment = HashSet<char>;

#[cfg(feature = "day8")]
pub fn part2(source: String) -> usize {
    source
    .split("\r\n")
    .map(
        |line| {
            let mut parts = line.split(" | ").map(|part| part.split_ascii_whitespace().map(|word| word.chars().collect::<Segment>()).collect::<Vec<_>>());
            let mut clues = parts.next().unwrap();
            let result = parts.next().unwrap();

            clues.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
            let one = &clues[0]; 
            let seven = &clues[1];
            let four = &clues[2];
            let eight = &clues[clues.len() - 1];
            let three = clues.iter().filter(|segment| segment.len() == 5 && segment.is_superset(one)).next().unwrap();
            let nine = clues.iter().filter(|segment| segment.len() == 6 && segment.is_superset(one) && segment.is_superset(three) ).next().unwrap();
            let reverse_one = eight - three;
            let zero = &(&(nine - four) | one) | &reverse_one;
            let bottom_left = &reverse_one - four;
            let top_left = &reverse_one - &bottom_left;
            let five = clues.iter().filter(|segment| segment.len() == 5 && segment.is_superset(&top_left)).next().unwrap();
            let two = clues.iter().filter(|segment| segment.len() == 5 && segment.is_superset(&bottom_left)).next().unwrap();
            let six = five | &bottom_left;  
            let segment_solution = [&zero, one, two, three, four, five, &six, seven, eight, nine];

            result
            .iter()
            .rev()
            .enumerate()
            .map(
                |(i, number)| segment_solution.iter().position(|seg| seg == &number).unwrap() * 10usize.pow(i as u32)
            ).sum::<usize>()
        }
    ).sum::<usize>()
}
