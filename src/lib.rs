use std::fs;

mod day8;

fn read_input_full(day: &str) -> String {
    fs::read_to_string(format!("input_{}.txt", day)).expect("Can't read")
}

fn read_input_small(day: &str) -> String {
    fs::read_to_string(format!("input_{}_small.txt", day)).expect("Can't read")
}
