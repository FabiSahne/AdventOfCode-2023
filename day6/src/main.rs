use std::fs::read_to_string;

fn main() {
    let data = read_to_string("C:/Users/Fabian/Code/AdventOfCode/2023/day6/input").unwrap();
    let lines: Vec<&str> = data.split('\n').filter(|x| !x.is_empty()).collect();

    let (strtimes, strdistances): (Vec<&str>, Vec<&str>) =
        (lines[0].split(':').collect(), lines[1].split(':').collect());

    let (times, distances): (Vec<i32>, Vec<i32>) = (
        strtimes[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        strdistances[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
    );

    // Part 1

    let mut possibles = vec![];

    for i in 0..times.len() {
        possibles.push(0);
        for time in 0..times[i] {
            if time * (times[i] - time) > distances[i] {
                possibles[i] += 1;
            }
        }
    }
    let mut result = 1;
    possibles.iter().for_each(|x| result *= x);
    println!("{result}");

    let (time, distance): (i64, i64) = (
        strtimes[1]
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap(),
        strdistances[1]
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap(),
    );

    // Part 2

    let mut result = 0;
    for i in 0..time {
        if i * (time - i) > distance {
            result += 1;
        }
    }
    println!("{result}");
}
