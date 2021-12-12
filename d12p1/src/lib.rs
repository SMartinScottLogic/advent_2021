use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> Solution {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut i = line.trim().split('-').map(|v| v.to_string());
        let a = i.next().unwrap();
        let b = i.next().unwrap();
        solution.add(a, b);
    }

    solution
}

#[derive(Debug, Default)]
pub struct Solution {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add(&mut self, a: String, b: String) {
        (*self.edges.entry(a.clone()).or_default()).insert(b.clone());
        (*self.edges.entry(b.clone()).or_default()).insert(a.clone());
        self.nodes.insert(a);
        self.nodes.insert(b);
    }

    fn walk(
        &self,
        current_node: String,
        mut path: Vec<String>,
        mut has_visited: HashSet<String>,
    ) -> i64 {
        let mut answer = 0;
        has_visited.insert(current_node.clone());
        path.push(current_node.clone());
        if current_node == "end".to_string() {
            answer += 1;
            println!("{} {:?}", current_node, path);
        } else {
            for next_node in &self.nodes {
                let current = current_node.clone();
                if *next_node == current {
                    continue;
                }
                let edges = self
                    .edges
                    .get(&current)
                    .map(|e| e.clone())
                    .unwrap_or_default();
                if !edges.contains(next_node) {
                    continue;
                }
                if next_node.contains(char::is_uppercase) || !has_visited.contains(next_node) {
                    let visit = has_visited.clone();
                    let path = path.clone();
                    answer += self.walk(next_node.clone(), path, visit);
                }
            }
        }
        answer
    }

    pub fn analyse(&mut self) {
        self.answer = self.walk("start".into(), Vec::new(), HashSet::new());

        println!("{}", self.answer);
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}
