use std::collections::HashMap;
use std::fs;

fn day1() -> u32 {
    let contents = fs::read_to_string("input_day1.txt").expect("Can't read");
    let mut previous = u32::MAX;
    let mut count = 0;
    for line in contents.lines() {
        let current = line.parse::<u32>().unwrap();
        if current > previous {
            count += 1;
        }
        previous = current;
    }
    count
}

fn day1_pt2() -> u32 {
    let mut lines = Vec::new();
    let contents = fs::read_to_string("input_day1.txt").expect("Can't read");
    for line in contents.lines() {
        lines.push(line.parse::<u32>().unwrap())
    }
    let mut previous = u32::MAX;
    let mut count = 0;
    for w in lines.as_slice().windows(3) {
        let window_sum = w.iter().sum::<u32>();
        if window_sum > previous {
            count += 1;
        }
        previous = window_sum;
    }
    count
}

fn day2_pt1() -> u32 {
    let contents = fs::read_to_string("input_day2.txt").expect("Can't read");
    let mut start_pos = (0, 0);
    for line in contents.lines() {
        let (command, value) = line.split_at(line.find(" ").unwrap());
        let value = value.trim().parse::<u32>().unwrap();
        match command {
            "forward" => start_pos.0 += value,
            "down" => start_pos.1 += value,
            "up" => start_pos.1 -= value,
            _ => (),
        }
    }
    start_pos.0 * start_pos.1
}

fn day2_pt2() -> u32 {
    let contents = fs::read_to_string("input_day2.txt").expect("Can't read");
    let (mut longitude, mut latitude, mut aim) = (0, 0, 0);
    for line in contents.lines() {
        let (command, value) = line.split_at(line.find(" ").unwrap());
        let value = value.trim().parse::<u32>().unwrap();
        match command {
            "forward" => {
                longitude += value;
                latitude += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => (),
        }
    }
    longitude * latitude
}

fn _gamma(gamma_binary: &[u32]) -> u32 {
    u32::from_str_radix(
        &gamma_binary
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn _epsilon(gamma_binary: &[u32]) -> u32 {
    u32::from_str_radix(
        &gamma_binary
            .into_iter()
            .map(|i| match i {
                0 => 1,
                1 => 0,
                _ => -1,
            })
            .map(|i| i.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn _count_occurences(contents: &String) -> HashMap<(usize, u32), u32> {
    let mut counts = HashMap::new();
    for line in contents.lines() {
        for (position, value) in line.chars().enumerate() {
            let value = value.to_digit(10).unwrap();
            *counts.entry((position, value)).or_insert(0) += 1;
        }
    }
    counts
}

fn _get_gamma_binary(counts: HashMap<(usize, u32), u32>) -> Vec<u32> {
    let mut gamma_binary = Vec::new();
    for i in 0..12 {
        let num_ones = *counts.get(&(i, 1)).unwrap();
        let num_zeros = *counts.get(&(i, 0)).unwrap();
        if num_ones >= num_zeros {
            gamma_binary.push(1);
        } else {
            gamma_binary.push(0);
        }
    }
    gamma_binary
}

fn day3_pt1() -> u32 {
    let contents = fs::read_to_string("input_day3.txt").expect("Can't read");
    let counts = _count_occurences(&contents);
    let gamma_binary = _get_gamma_binary(counts);
    _gamma(gamma_binary.as_slice()) * _epsilon(gamma_binary.as_slice())
}

fn _find_oxygen_rating(contents: Vec<&str>, idx: usize) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut num_ones = 0;
    for line in contents.iter() {
        num_ones += line.chars().nth(idx).unwrap().to_digit(10).unwrap();
    }
    let desired_c;
    match num_ones >= (contents.len() / 2).try_into().unwrap() {
        true => {
            desired_c = '1';
        }
        false => {
            desired_c = '0';
        }
    }
    for line in contents.iter() {
        if line.chars().nth(idx).unwrap() == desired_c {
            result.push(line);
        }
    }
    result
}

fn _find_co2_rating(contents: Vec<&str>, idx: usize) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut num_ones = 0;
    for line in contents.iter() {
        num_ones += line.chars().nth(idx).unwrap().to_digit(10).unwrap();
    }
    let desired_c;
    match num_ones < (contents.len() / 2).try_into().unwrap() {
        true => {
            desired_c = '1';
        }
        false => {
            desired_c = '0';
        }
    }
    for line in contents.iter() {
        if line.chars().nth(idx).unwrap() == desired_c {
            result.push(line);
        }
    }
    result
}

fn day3_pt2() -> u32 {
    let contents = fs::read_to_string("input_day3.txt").expect("Can't read");
    let contents: Vec<&str> = contents.lines().collect();
    let mut start_idx = 0;
    let mut oxygen = _find_oxygen_rating(contents.clone(), start_idx);
    while oxygen.len() > 1 {
        start_idx += 1;
        oxygen = _find_oxygen_rating(oxygen.clone(), start_idx);
    }
    let oxy = u32::from_str_radix(oxygen[0], 2).unwrap();
    start_idx = 0;
    let mut co2 = _find_co2_rating(contents.clone(), start_idx);
    while co2.len() > 1 {
        start_idx += 1;
        co2 = _find_co2_rating(co2.clone(), start_idx);
    }
    let co2r = u32::from_str_radix(co2[0], 2).unwrap();
    oxy * co2r
}

fn main() {
    //println!("Day1 problem {}", day1());
    //println!("Day 2 problem {}", day1_pt2());
    //println!("{}", day2_pt1());
    //println!("{}", day2_pt2());
    println!("pt1: {}", day3_pt1());
    println!("pt2: {}", day3_pt2());
}
