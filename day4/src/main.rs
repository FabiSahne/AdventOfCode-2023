use std::fs::read_to_string;

fn main() {
    let mut result = 0;
    let mut copy = vec![];
    if let Ok(data) = read_to_string("C:/Users/Fabian/Code/AdventOfCode/2023/day4/input") {
        let mut lines: Vec<&str> = data.split("\n").collect();
        lines.pop();
        for line in lines {
            copy.push(line);
        }
        loop {
            if result < copy.len() {
                let line = copy[result];
                let game: Vec<&str> = line.split(": ").collect();
                let card: Vec<&str> = game[1].split("| ").collect();
                let winning: Vec<i32> = card[0]
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                let numbers: Vec<i32> = card[1]
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                let mut matching = 0;
                for n in numbers {
                    if winning.contains(&n) {
                        matching += 1;
                        copy.push(copy[result + matching]);
                    }
                }
                result += 1;
            } else {
                break;
            }
        }
        result = copy.len();
    }
    println!("{result}");
}
