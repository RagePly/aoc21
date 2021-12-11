// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day10");

#[cfg(not(feature = "day10"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(10, 1)
}

#[cfg(not(feature = "day10"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(10, 2)
}

// v START SOLUTION v
#[cfg(feature = "day10")]
const OPENING: [char; 4] = ['(', '[', '{', '<'];
#[cfg(feature = "day10")]
const CLOSING: [char; 4] = [')', ']', '}', '>'];
#[cfg(feature = "day10")]
const POINTS: [usize; 4] = [3, 57, 1197, 25137];

#[cfg(feature = "day10")]
#[inline]
fn matching_closing(braket: char) -> char {
    CLOSING[OPENING.iter().position(|&c| c == braket).unwrap()]
}

#[cfg(feature = "day10")]
pub fn part1(source: String) -> usize {
    source
    .split("\r\n")
    .map(
        |line| {
            let mut stack: Vec<char> = Vec::new();
            for braket in line.chars() {
                if OPENING.contains(&braket) {
                    stack.push(braket);
                } else if matching_closing(stack.pop().unwrap()) != braket {
                    return POINTS[CLOSING.iter().position(|&c| c == braket).unwrap()];
                }
            }
            return 0;
        }
    )
    .sum()
}

#[cfg(feature = "day10")]
pub fn part2(source: String) -> i64 {
    let mut scores: Vec<i64> = source
    .split("\r\n")
    .map(
        |line| {
            let mut stack: Vec<char> = Vec::new();
            for braket in line.chars() {
                if OPENING.contains(&braket) {
                    stack.push(braket);
                } else if matching_closing(stack.pop().unwrap()) != braket {
                    return -1; 
                }
            }

            stack
            .into_iter()
            .rev()
            .fold(0, 
                |state, braket| state * 5 + OPENING.iter().position(|&c| c == braket).unwrap() as i64 + 1 
            )
        }
    )
    .filter(|&score| score != -1)
    .collect();
    
    scores.sort_unstable();
    scores[scores.len() / 2]
}
