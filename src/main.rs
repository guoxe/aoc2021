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

fn day3_pt1() -> u32 {
    let contents = fs::read_to_string("input_day3.txt").expect("Can't read");
    let mut counts = HashMap::new();
    for line in contents.lines() {
        for (position, value) in line.chars().enumerate() {
            let value = value.to_digit(10).unwrap();
            *counts.entry((position, value)).or_insert(0) += 1;
        }
    }
    let mut gamma_binary = Vec::new();
    for i in 0..12 {
        let num_ones = *counts.get(&(i, 1)).unwrap();
        let num_zeros = *counts.get(&(i, 0)).unwrap();
        if num_ones > num_zeros {
            gamma_binary.push(1);
        } else {
            gamma_binary.push(0);
        }
    }
    let gamma = u32::from_str_radix(
        &gamma_binary
            .clone()
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();
    let epsilon = u32::from_str_radix(
        &gamma_binary
            .clone()
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
    .unwrap();
    gamma * epsilon
}

fn main() {
    //println!("Day1 problem {}", day1());
    //println!("Day 2 problem {}", day1_pt2());
    //println!("{}", day2_pt1());
    //println!("{}", day2_pt2());
    println!("{}", day3_pt1());
}
