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

fn main() {
    println!("Final count {}", day1());
}
