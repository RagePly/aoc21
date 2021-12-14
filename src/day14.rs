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
type ScoreChache = HashMap<Pair, Score>;

#[cfg(feature = "day14")]
type LevelCache = Vec<ScoreChache>;

#[cfg(feature = "day14")]
type Insertions = HashMap<Pair, char>;

#[cfg(feature = "day14")]
fn scoring_recursive(
    pair: Pair, 
    insertions: &Insertions,
    level_cache: &mut LevelCache,
    depth: usize,
) {
    let mut scoring = Score::new();
    let new = insertions[&pair];
    scoring.insert(new, 1);

    if depth != 0 {
        [[pair[0], new], [new, pair[1]]]
        .iter()
        .for_each(|new_pair| {
            if level_cache.len() <= depth - 1 || !level_cache[depth - 1].contains_key(new_pair) {
                // is guaranteed to insert new_pair into 
                scoring_recursive(*new_pair, insertions, level_cache, depth - 1);
            }

            level_cache[depth - 1][new_pair]
            .iter()
            .for_each(|(c, s)| *scoring.entry(*c).or_insert(0) += s);
        });
    }

    if level_cache.len() == depth { 
        let mut score_cache = ScoreChache::new();
        score_cache.insert(pair, scoring);
        level_cache.push(score_cache);
    } else {
        level_cache[depth].insert(pair, scoring);
    }
}

#[cfg(feature = "day14")]
pub fn part1(source: String) -> usize {
    let mut parts = source.split("\r\n\r\n");
    let orig_state: Vec<_> = parts.next().unwrap().chars().collect();
    let mapping: Insertions = parts
    .next()
    .unwrap()
    .split("\r\n")
    .map(|line| {
        let mut terms = line.split(" -> ");
        let mut temp = terms.next().unwrap().chars();
        let from: Pair = [temp.next().unwrap(), temp.next().unwrap()];
        let to: char = terms.next().unwrap().chars().next().unwrap();
        (from, to)
    })
    .collect();

    // apply recursive function
    let mut level_cache = LevelCache::new(); 
    orig_state
    .as_slice()
    .windows(2)
    .for_each(|v|
        scoring_recursive([v[0], v[1]], &mapping, &mut level_cache, 9)
    );

    // add accumulated scores
    let mut scoring = Score::new();
    level_cache[9]
    .iter()
    .for_each(|(_, scores)| 
        scores
        .iter()
        .for_each(|(c, s)| 
            *scoring.entry(*c).or_insert(0) += s
        )
    );

    // add original letters to score
    orig_state
    .iter()
    .for_each(|&c| 
        *scoring.entry(c).or_insert(0) += 1
    );

    // collect scores and sort
    let mut scores: Vec<_> = scoring.values().map(|&v| v).collect();
    scores.sort_unstable();
    scores[scores.len() - 1] - scores[0]
}

#[cfg(feature = "day14")]
pub fn part2(source: String) -> usize {
    let mut parts = source.split("\r\n\r\n");
    let orig_state: Vec<_> = parts.next().unwrap().chars().collect();
    let mapping: Insertions = parts
    .next()
    .unwrap()
    .split("\r\n")
    .map(|line| {
        let mut terms = line.split(" -> ");
        let mut temp = terms.next().unwrap().chars();
        let from: Pair = [temp.next().unwrap(), temp.next().unwrap()];
        let to: char = terms.next().unwrap().chars().next().unwrap();
        (from, to)
    })
    .collect();

    // apply recursive function
    let mut level_cache = LevelCache::new(); 
    orig_state
    .as_slice()
    .windows(2)
    .for_each(|v|
        scoring_recursive([v[0], v[1]], &mapping, &mut level_cache, 39)
    );

    // add accumulated scores
    let mut scoring = Score::new();
    level_cache[39]
    .iter()
    .for_each(|(_, scores)| 
        scores
        .iter()
        .for_each(|(c, s)| 
            *scoring.entry(*c).or_insert(0) += s
        )
    );

    // add original letters to score
    orig_state
    .iter()
    .for_each(|&c| 
        *scoring.entry(c).or_insert(0) += 1
    );

    // collect scores and sort
    let mut scores: Vec<_> = scoring.values().map(|&v| v).collect();
    scores.sort_unstable();
    scores[scores.len() - 1] - scores[0]
}
