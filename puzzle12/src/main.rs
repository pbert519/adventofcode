use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let lines = BufReader::new(file).lines();
    let edges = lines
        .map(|line| {
            let line = line.unwrap();
            let mut splited = line.split('-');
            (
                splited.next().unwrap().to_string(),
                splited.next().unwrap().to_string(),
            )
        })
        .collect::<Vec<(String, String)>>();

    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for edge in &edges {
        graph.entry(&edge.0).or_insert(Vec::new()).push(&edge.1);
        graph.entry(&edge.1).or_insert(Vec::new()).push(&edge.0);
    }

    let path_count = visit_neighbours(&graph, "start", Vec::new());
    println!("{}", path_count);
}

fn visit_neighbours(graph: &BTreeMap<&str, Vec<&str>>, node: &str, visited: Vec<&str>) -> i32 {
    return if node.contains("end") {
        1
    } else if check_enter_cave(&visited, node) {
        let mut paths = 0;
        for n in &graph[node] {
            let mut clone = visited.clone();
            clone.push(node);
            paths += visit_neighbours(graph, n, clone)
        }
        paths
    } else {
        0
    };
}

fn check_enter_cave(visited: &[&str], node: &str) -> bool {
    let number_of_visits = visited.iter().filter(|x| node.contains(**x)).count();
    let uppercase = node.chars().next().unwrap().is_ascii_uppercase();
    if uppercase || number_of_visits < 1 {
        true
    } else if !node.contains("start") {
        // set of all nodes
        let set = visited
            .iter()
            .filter(|node| node.chars().next().unwrap().is_ascii_lowercase())
            .collect::<HashSet<_>>();
        for s in set {
            let number_of_visits = visited.iter().filter(|x| s.contains(**x)).count();
            if number_of_visits > 1 {
                return false;
            }
        }
        true
    } else {
        false
    }
}
