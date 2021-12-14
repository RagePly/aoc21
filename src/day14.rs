// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day14");

#[cfg(not(feature = "day14"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(14, 1)
}

#[cfg(not(feature = "day14"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(14, 2)
}

// v START SOLUTION v
#[cfg(feature = "day14")]
use std::collections::HashMap;

#[cfg(feature = "day14")]
fn recurse(left: char, right: char, depth: usize, max_depth: usize, mapping: &HashMap<[char; 2], char>, scores: &mut HashMap<char, usize>) {
    if depth > max_depth {
        return
    } else {
        // get new
        let new = mapping[&[left, right]];
        *scores.entry(new).or_insert(0) += 1;
        recurse(left, new, depth + 1, max_depth, mapping, scores);
        recurse(new, right, depth + 1, max_depth, mapping, scores);
    }
}

#[cfg(feature = "day14")]
pub fn part1(source: String) -> usize {
    let mut parts = source.split("\r\n\r\n");
    let orig_state: Vec<_> = parts.next().unwrap().chars().collect();
    let mapping: HashMap<[char; 2], char> = parts.next().unwrap()
    .split("\r\n")
    .map(|line| {
        let mut terms = line.split(" -> ");
        let mut temp = terms.next().unwrap().chars();
        let from = [temp.next().unwrap(), temp.next().unwrap()];
        let to = terms.next().unwrap().chars().next().unwrap();
        (from, to)
    })
    .collect();

    let mut scoring: HashMap<char, usize> = HashMap::new();
    orig_state
    .iter()
    .for_each(|&c| *scoring.entry(c).or_insert(0) += 1);

    orig_state
    .as_slice()
    .windows(2)
    .for_each(|v|
        recurse(v[0], v[1], 1, 10, &mapping, &mut scoring)
    );

    let mut scores: Vec<_> = scoring.values().map(|&v| v).collect();
    scores.sort_unstable();
    scores[scores.len() - 1] - scores[0]
}

#[cfg(feature = "day14")]
pub fn part2(source: String) -> String {
    String::from("not implemented")
}
