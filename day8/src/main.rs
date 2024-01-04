mod Solution;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let data = read_to_string("C:\\Users\\Fabian\\Code\\AdventOfCode\\2023\\day8\\input").unwrap();

    let lines: Vec<&str> = data.split('\n').filter(|&x| !x.is_empty()).collect();

    let instruction = lines[0];
    let ins_size = instruction.len();

    let mut nodes = HashMap::new();

    for line in lines {
        if line.contains("=") {
            let (src, dst) = line.split_once(" = ").unwrap();
            let (dst_left, dst_right) = dst.split_once(", ").unwrap();
            let dst_left = dst_left
                .chars()
                .filter(|x| x.is_alphabetic())
                .collect::<String>();
            let dst_right = dst_right
                .chars()
                .filter(|x| x.is_alphabetic())
                .collect::<String>();

            nodes.insert(src.to_uppercase(), (dst_left, dst_right));
        }
    }

    let end_a = Regex::new("[A-Z]{2}A").unwrap();
    let end_z = Regex::new("[A-Z]{2}Z").unwrap();

    let starting_nodes: HashMap<String, (String, String)> = nodes
        .clone()
        .into_iter()
        .filter(|(k, _v)| end_a.is_match(k))
        .collect();

    let mut positions: HashSet<String> = starting_nodes.keys().cloned().collect();
    let mut steps = 0;

    while !positions.iter().all(|x| x.chars().nth(2).unwrap() == 'Z') {
        let next_dir = instruction.chars().nth(steps % ins_size).unwrap();
        let next_pos: Vec<(String, String)> = nodes
            .clone()
            .into_iter()
            .filter(|(k, _v)| positions.contains(k))
            .map(|(_k, v)| v)
            .collect();
        positions.clear();
        if next_dir == 'L' {
            for (l, _r) in &next_pos {
                positions.insert(l.clone());
            }
        }
        if next_dir == 'R' {
            for (_l, r) in &next_pos {
                positions.insert(r.clone());
            }
        }
        steps += 1;
        if steps % 100_000 == 0 {
            println!("{steps}");
        }
    }

    println!("{steps}");
}
