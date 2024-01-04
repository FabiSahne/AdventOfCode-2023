use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut result = 0;
    if let Ok(lines) = read_lines("C:/Users/Fabian/Code/AdventOfCode/2023/day2/input") {
        for line in lines {
            if let Ok(text) = line {
                let mut power = 0;
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                let game: Vec<&str> = text.split(": ").collect();
                //let _id = game[0].split(" ").find(|&x| x.parse::<i32>().is_ok()).unwrap().parse::<i32>().unwrap();
                let games: Vec<&str> = game[1].split("; ").collect();
                for turn in games {
                    let cubes: Vec<&str> = turn.split(", ").collect();
                    for cube in cubes {
                        let subset: Vec<&str> = cube.split(" ").collect();
                        let num = subset[0].parse::<i32>().unwrap();
                        match subset[1] {
                            "green" => {if num > green {green = num;}},
                            "red" => {if num > red {red = num;}},
                            "blue" => {if num > blue {blue = num;}},
                            _ => ()
                        }
                    }
                    power = green*red*blue;
                }
                result += power;
            }
        }
    }
    println!("{result}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}