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
use std::rc::Rc;

#[cfg(feature = "day12")]
type NodeGraph = HashMap<Rc<String>, Vec<Rc<String>>>;


#[cfg(feature = "day12")]
type PathStack = Vec<Rc<String>>;

#[cfg(feature = "day12")]
#[inline]
fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

#[cfg(feature = "day12")]
fn dfs(stack: &mut PathStack, graph: &NodeGraph, twice: bool) -> usize {
    let top = stack.last().unwrap().clone();
    graph[&top]
    .iter()
    .map(|node| if *node.as_ref() == String::from("end") {
            1
        } else if *node.as_ref() == String::from("start") {
            0
        } else if is_uppercase(node) || !stack.contains(node) {
            stack.push(node.clone());
            let count = dfs(stack, graph, twice);
            stack.pop();
            count
        } else if !twice {
            stack.push(node.clone());
            let count = dfs(stack, graph, !twice);
            stack.pop();
            count
        } else {
            0
        }
    )
    .sum()
}

#[cfg(feature = "day12")]
pub fn part1(source: String) -> usize {
    let mut graph = NodeGraph::new();
    source
    .split("\r\n")
    .for_each(
        |line| {
            let mut nodes = line.split("-");
            let node1 = Rc::new(String::from(nodes.next().unwrap()));
            let node2 = Rc::new(String::from(nodes.next().unwrap()));
            graph.entry(node1.clone()).or_insert(Vec::new()).push(node2.clone());
            graph.entry(node2).or_insert(Vec::new()).push(node1);
        }
    );

    let mut stack = vec!(graph.get_key_value(&String::from("start")).unwrap().0.clone());
    dfs(&mut stack, &graph, true)
}

#[cfg(feature = "day12")]
pub fn part2(source: String) -> usize {
    let mut graph = NodeGraph::new();
    source
    .split("\r\n")
    .for_each(
        |line| {
            let mut nodes = line.split("-");
            let node1 = Rc::new(String::from(nodes.next().unwrap()));
            let node2 = Rc::new(String::from(nodes.next().unwrap()));
            graph.entry(node1.clone()).or_insert(Vec::new()).push(node2.clone());
            graph.entry(node2).or_insert(Vec::new()).push(node1);
        }
    );

    let mut stack = vec!(graph.get_key_value(&String::from("start")).unwrap().0.clone());
    dfs(&mut stack, &graph, false)
}
