// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day12");

#[cfg(not(feature = "day12"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(12, 1)
}

#[cfg(not(feature = "day12"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(12, 2)
}

// v START SOLUTION v
#[cfg(feature = "day12")]
use std::collections::HashMap;

#[cfg(feature = "day12")]
type Uppercases = Vec<bool>;

#[cfg(feature = "day12")]
type NodeGraph = Vec<Vec<usize>>;

#[cfg(feature = "day12")]
type PathStack = Vec<bool>;

#[cfg(feature = "day12")]
#[inline]
fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

#[cfg(feature = "day12")]
fn dfs(top: usize, stack: &mut PathStack, graph: &NodeGraph, twice: bool, uppercase: &Uppercases, start: usize, end: usize) -> usize {
    graph[top]
    .iter()
    .map(|&node| if node == end {
            1
        } else if node == start {
            0
        } else if uppercase[node] || !stack[node] {
            stack[node] = true;
            let count = dfs(node, stack, graph, twice, uppercase, start, end);
            stack[node] = false;
            count
        } else if !twice {
            dfs(node, stack, graph, !twice, uppercase, start, end)
        } else {
            0
        }
    )
    .sum()
}

#[cfg(feature = "day12")]
pub fn part1(source: String) -> usize {
    let mut graph = NodeGraph::new();
    let mut index_mapping: HashMap<String, usize> = HashMap::new();
    let mut uppercase: Uppercases = Uppercases::new();
    let mut iota = 0;
    source
    .split("\r\n")
    .for_each(
        |line| {
            let mut is = line
            .split("-")
            .map(|node| {
                match index_mapping.get(node) {
                    Some(&i) => i,
                    None => {
                        uppercase.push(is_uppercase(node));
                        graph.push(Vec::new());
                        index_mapping.insert(String::from(node), iota);
                        iota += 1;
                        iota - 1
                    }
                }
            });

            let i1 = is.next().unwrap();
            let i2 = is.next().unwrap();
            graph[i1].push(i2);
            graph[i2].push(i1);
        }
    );
    
    let start = index_mapping["start"];
    let end = index_mapping["end"];
    let mut stack = (0..graph.len()).map(|_| false).collect();
    dfs(start, &mut stack, &graph, true, &uppercase, start, end)
}

#[cfg(feature = "day12")]
pub fn part2(source: String) -> usize {
    let mut graph = NodeGraph::new();
    let mut index_mapping: HashMap<String, usize> = HashMap::new();
    let mut uppercase: Uppercases = Uppercases::new();
    let mut iota = 0;
    source
    .split("\r\n")
    .for_each(
        |line| {
            let mut is = line
            .split("-")
            .map(|node| {
                match index_mapping.get(node) {
                    Some(&i) => i,
                    None => {
                        uppercase.push(is_uppercase(node));
                        graph.push(Vec::new());
                        index_mapping.insert(String::from(node), iota);
                        iota += 1;
                        iota - 1
                    }
                }
            });

            let i1 = is.next().unwrap();
            let i2 = is.next().unwrap();
            graph[i1].push(i2);
            graph[i2].push(i1);
        }
    );
    
    let start = index_mapping["start"];
    let end = index_mapping["end"];
    let mut stack = (0..graph.len()).map(|_| false).collect();
    dfs(start, &mut stack, &graph, false, &uppercase, start, end)
}
