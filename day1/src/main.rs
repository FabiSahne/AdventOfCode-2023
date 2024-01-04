use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut result = 0;
    if let Ok(lines) = read_lines("C:/Users/Fabian/Code/AdventOfCode/2023/day1/input") {
        for line in lines {
            let mut calib_nr = 0;
            if let Ok(mut text) = line {
                text = text.replace("one", "1");
                text = text.replace("two", "2");
                text = text.replace("three", "3");
                text = text.replace("four", "4");
                text = text.replace("five", "5");
                text = text.replace("six", "6");
                text = text.replace("seven", "7");
                text = text.replace("eight", "8");
                text = text.replace("nine", "9");
                calib_nr += text
                    .chars()
                    .find(|x| x.is_numeric())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                calib_nr *= 10;
                calib_nr += text
                    .chars()
                    .filter(|x| x.is_numeric())
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
            }
            result += calib_nr;
        }
    }
    println!("{result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
