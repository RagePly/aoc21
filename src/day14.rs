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
type Pair = [char; 2];

#[cfg(feature = "day14")]
type Score = HashMap<char, usize>;

#[cfg(feature = "day14")]
type PairCount = HashMap<Pair, usize>;


#[cfg(feature = "day14")]
type Insertions = HashMap<Pair, char>;

#[cfg(feature = "day14")]
fn parse(source: String) -> (Vec<char>, Insertions) {
    let mut parts = source.split("\r\n\r\n");
    let orig_state: Vec<_> = parts.next().unwrap().chars().collect();
    let mapping: Insertions = parts
    .next()
    .unwrap()
    .split("\r\n")
    .map(|line| {
        let terms: Vec<char> = line.split(" -> ").map(|t| t.chars()).flatten().collect();
        ([terms[0], terms[1]], terms[2])
    })
    .collect();

    (orig_state, mapping)
}

#[cfg(feature = "day14")]
fn run(orig_state: Vec<char>, mapping: Insertions, steps: usize) -> usize {
    let mut score = Score::new();
    orig_state.iter().for_each(|&c| *score.entry(c).or_insert(0) += 1);

    let mut count = PairCount::new();
    orig_state.as_slice().windows(2).for_each(|v| *count.entry([v[0], v[1]]).or_insert(0) += 1);

    for _ in 0..steps {
        let mut new_count = PairCount::new();
        for (pair, v) in count {
            let new = mapping[&pair];
            *score.entry(new).or_insert(0) += v;
            *new_count.entry([pair[0], new]).or_insert(0) += v;
            *new_count.entry([new, pair[1]]).or_insert(0) += v;
        }

        count = new_count;
    }

    let mut scores: Vec<_> = score.values().collect();
    scores.sort_unstable();
    scores[scores.len() -1] - scores[0]
}


#[cfg(feature = "day14")]
pub fn part1(source: String) -> usize {
    let (orig_state, mapping) = parse(source);
    run(orig_state, mapping, 10)
}


#[cfg(feature = "day14")]
pub fn part2(source: String) -> usize {
    let (orig_state, mapping) = parse(source);
    run(orig_state, mapping, 40)
}
