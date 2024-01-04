use std::fs::read_to_string;

fn main() {
    let data = read_to_string("C:/Users/Fabian/Code/AdventOfCode/2023/day5/input").unwrap();
    let lines: Vec<&str> = data.split('\n').filter(|x| !x.is_empty()).collect();

    let seednums: Vec<i64> = lines[0]
        .split_whitespace()
        .filter(|&x| x.parse::<i64>().is_ok())
        .map(|x| x.parse().unwrap())
        .collect();

    let seed2soilid = lines
        .iter()
        .position(|&x| x.contains("seed-to-soil"))
        .unwrap();
    let soil2fertid = lines
        .iter()
        .position(|&x| x.contains("soil-to-fertilizer"))
        .unwrap();
    let fert2waterid = lines
        .iter()
        .position(|&x| x.contains("fertilizer-to-water"))
        .unwrap();
    let water2lightid = lines
        .iter()
        .position(|&x| x.contains("water-to-light"))
        .unwrap();
    let light2tempid = lines
        .iter()
        .position(|&x| x.contains("light-to-temperature"))
        .unwrap();
    let temp2humidid = lines
        .iter()
        .position(|&x| x.contains("temperature-to-humidity"))
        .unwrap();
    let humid2locatid = lines
        .iter()
        .position(|&x| x.contains("humidity-to-location"))
        .unwrap();

    let s2smap = &lines[seed2soilid + 1..soil2fertid].to_vec();
    let s2fmap = &lines[soil2fertid + 1..fert2waterid].to_vec();
    let f2wmap = &lines[fert2waterid + 1..water2lightid].to_vec();
    let w2lmap = &lines[water2lightid + 1..light2tempid].to_vec();
    let l2tmap = &lines[light2tempid + 1..temp2humidid].to_vec();
    let t2hmap = &lines[temp2humidid + 1..humid2locatid].to_vec();
    let h2lmap = &lines[humid2locatid + 1..].to_vec();

    let mut location = -1;
    let mut seed = 0;

    while !seedfound(seed, &seednums) {
        location += 1;
        let humid = corr(h2lmap, location);
        let temp = corr(t2hmap, humid);
        let light = corr(l2tmap, temp);
        let water = corr(w2lmap, light);
        let fert = corr(f2wmap, water);
        let soil = corr(s2fmap, fert);
        seed = corr(s2smap, soil);
    }

    println!("{location}");
}

fn seedfound(seed: i64, seednums: &Vec<i64>) -> bool {
    let mut found = false;

    for i in 0..seednums.len() / 2 {
        if (seednums[i * 2]..seednums[i * 2] + seednums[i * 2 + 1]).contains(&seed) {
            found = true;
        }
    }

    return found;
}
fn corr(map: &Vec<&str>, dst: i64) -> i64 {
    for &s in map {
        let s: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if (s[0]..s[0] + s[2]).contains(&dst) {
            return s[1] + dst - s[0];
        }
    }

    return dst;
}
