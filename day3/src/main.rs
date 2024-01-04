use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let mut result1 = 0;
    let mut result2 = 0;
    if let Ok(data_raw) = read_to_string("C:/Users/Fabian/Code/AdventOfCode/2023/day3/input") {
        let width = data_raw.split_once("\n").unwrap().0.len();
        let data: Vec<char> = data_raw.replace("\n", "").chars().collect();

        let mut pos_to_label = HashMap::new();
        let mut label_to_value = HashMap::new();
        let mut gear_pos = vec![];
        let mut sybol_pos = vec![];

        fn update(
            label: &mut usize,
            num: &mut Vec<usize>,
            pos_to_label: &mut HashMap<usize, usize>,
            label_to_value: &mut HashMap<usize, usize>,
            data: &[char],
        ) {
            if !num.is_empty() {
                pos_to_label.extend(num.iter_mut().map(|p| (*p, *label)));
                label_to_value.insert(
                    *label,
                    num.iter().fold(0, |acc, elem| {
                        acc * 10 + data[*elem].to_digit(10).unwrap() as usize
                    }),
                );
                num.clear();
                *label += 1;
            }
        }

        let mut label = 0;
        let mut num = vec![];

        let mut start = 0;
        while start != data.len() {
            for (char, pos) in data[start..start + width].iter().zip(start..start + width) {
                if char.is_ascii_digit() {
                    num.push(pos);
                } else {
                    if *char == '*' {
                        gear_pos.push(pos)
                    }
                    if *char != '.' {
                        sybol_pos.push(pos)
                    }
                    update(
                        &mut label,
                        &mut num,
                        &mut pos_to_label,
                        &mut label_to_value,
                        &data,
                    );
                }
            }
            update(
                &mut label,
                num.as_mut(),
                &mut pos_to_label,
                &mut label_to_value,
                &data,
            );
            start += width;
        }
        result1 = sybol_pos
            .iter()
            .flat_map(|s| get_neighbor_labels(*s, width, &pos_to_label))
            .unique()
            .map(|l| label_to_value[l])
            .sum();

        result2 = gear_pos
            .iter()
            .map(|gear| get_neighbor_labels(*gear, width, &pos_to_label))
            .filter(|x| x.len() == 2)
            .map(|l| label_to_value[&l[0]] * label_to_value[&l[1]])
            .sum();
    }
    println!("{result1} and {result2}");
}

fn get_neighbor_labels(
    pos: usize,
    array_width: usize,
    pos_to_label: &HashMap<usize, usize>,
) -> Vec<&usize> {
    let x_idx = pos / array_width;
    let y_idx = pos % array_width;
    let mut neighbors = vec![];
    // left, right, up, down, ul, ur, ll, lr
    for (x, y) in [
        (x_idx - 1, y_idx),
        (x_idx + 1, y_idx),
        (x_idx, y_idx + 1),
        (x_idx, y_idx - 1),
        (x_idx - 1, y_idx + 1),
        (x_idx + 1, y_idx + 1),
        (x_idx - 1, y_idx - 1),
        (x_idx + 1, y_idx - 1),
    ] {
        if (0..array_width).contains(&x) && (y > 0) {
            let neighbor_pos = x * array_width + y;
            neighbors.push(pos_to_label.get(&neighbor_pos));
        }
    }
    neighbors
        .iter()
        .filter_map(|p| *p)
        .unique()
        .collect::<Vec<&usize>>()
}
