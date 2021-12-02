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

fn main() {
    println!("Day1 problem {}", day1());
    println!("Day 2 problem {}", day1_pt2());
}
