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
        extra_visit: Vec<String>,
    ) -> i64 {
        let mut answer = 0;
        has_visited.insert(current_node.clone());
        path.push(current_node.clone());
        if current_node == *"end" {
            answer += 1;
            println!("{} {:?}", current_node, path);
        } else {
            for next_node in &self.nodes {
                let current = current_node.clone();
                if *next_node == current {
                    continue;
                }
                let edges = self.edges.get(&current).cloned().unwrap_or_default();
                if !edges.contains(next_node) {
                    continue;
                }

                let visit = has_visited.clone();
                let path = path.clone();
                let mut extra = extra_visit.clone();
                let mut should_visit = false;

                if next_node.contains(char::is_uppercase) || !has_visited.contains(next_node) {
                    should_visit = true;
                } else if extra_visit.is_empty() && next_node != "start" && next_node != "end" {
                    should_visit = true;
                    extra.push(next_node.clone());
                }
                if should_visit {
                    answer += self.walk(next_node.clone(), path, visit, extra);
                }
            }
        }
        answer
    }

    pub fn analyse(&mut self) {
        self.answer = self.walk("start".into(), Vec::new(), HashSet::new(), Vec::new());

        println!("{}", self.answer);
    }

    pub fn answer(&self) -> i64 {
        self.answer as i64
    }
}
