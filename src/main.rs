use std::cmp;
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

#[derive(Debug)]
struct Entry {
    number: u32,
    drawn: bool,
}

impl Entry {
    fn from_number(i: u32) -> Entry {
        Entry {
            number: i,
            drawn: false,
        }
    }
    fn mark(&mut self) {
        self.drawn = true;
    }
}

#[derive(Debug)]
struct Board {
    data: Vec<Vec<Entry>>,
    won_at: Option<usize>,
    winning_score: Option<u32>,
}

impl Board {
    fn from_data(b: Vec<Vec<Entry>>) -> Board {
        Board {
            data: b,
            won_at: None,
            winning_score: None,
        }
    }

    fn mark(&mut self, i: u32) {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                if entry.number == i {
                    entry.mark();
                }
            }
        }
    }

    fn _check_row(&self, row_idx: usize) -> bool {
        self.data[row_idx].iter().map(|e| e.drawn as u8).sum::<u8>() == 5
    }

    fn _check_col(&self, col_idx: usize) -> bool {
        self.data
            .iter()
            .map(|col| &col[col_idx])
            .map(|e| e.drawn as u8)
            .sum::<u8>()
            == 5
    }
    fn is_winning(&self) -> bool {
        for i in 0..4 {
            if self._check_row(i) || self._check_col(i) {
                return true;
            }
        }
        false
    }

    fn score(&self, last_draw: u32) -> u32 {
        self.data
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|e| !e.drawn)
                    .map(|e| e.number)
                    .sum::<u32>()
            })
            .sum::<u32>()
            * last_draw
    }

    fn record_win(&mut self, draw: u32, round: usize) {
        match self.won_at {
            Some(_) => (),
            None => {
                self.won_at = Some(round);
                self.winning_score = Some(self.score(draw))
            }
        }
    }
}

fn day4_pt1() -> u32 {
    let input = fs::read_to_string("input_day4.txt").expect("failed to read");
    let mut lines = input.lines();
    let draw_numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    let filtered_input: Vec<&str> = lines.filter(|l| l.len() > 0).collect();
    let mut boards: Vec<Board> = filtered_input
        .as_slice()
        .chunks_exact(5)
        .map(|board| {
            board
                .iter()
                .map(|l| {
                    l.split(" ")
                        .map(|i| i.replace(" ", ""))
                        .filter(|i| !i.is_empty())
                        .map(|i| Entry::from_number(i.parse::<u32>().unwrap()))
                        .collect()
                })
                .collect()
        })
        .map(|b| Board::from_data(b))
        .collect();

    for i in draw_numbers {
        for board in boards.iter_mut() {
            board.mark(i);
            if board.is_winning() {
                return board.score(i);
            }
        }
    }
    0
}

fn day4_pt2() -> u32 {
    let input = fs::read_to_string("input_day4.txt").expect("failed to read");
    let mut lines = input.lines();
    let draw_numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    let filtered_input: Vec<&str> = lines.filter(|l| l.len() > 0).collect();
    let mut boards: Vec<Board> = filtered_input
        .as_slice()
        .chunks_exact(5)
        .map(|board| {
            board
                .iter()
                .map(|l| {
                    l.split(" ")
                        .map(|i| i.replace(" ", ""))
                        .filter(|i| !i.is_empty())
                        .map(|i| Entry::from_number(i.parse::<u32>().unwrap()))
                        .collect()
                })
                .collect()
        })
        .map(|b| Board::from_data(b))
        .collect();

    for (round, &number) in draw_numbers.iter().enumerate() {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.is_winning() {
                if board.score(number) > 0 {
                    board.record_win(number, round);
                }
            }
        }
    }
    match boards.iter().max_by_key(|b| match b.won_at {
        Some(i) => i,
        None => 0,
    }) {
        Some(b) => b.winning_score.unwrap(),
        None => 0,
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(input: &str) -> Point {
        let coords: Vec<i32> = input
            .split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        Point {
            x: coords[0],
            y: coords[1],
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn intersect(&self, p: &Point) -> bool {
        if self.start.x == self.end.x || self.start.y == self.end.y {
            return p.x >= self.start.x
                && p.x <= self.end.x
                && p.y >= self.start.y
                && p.y <= self.end.y;
        } else {
            let cross = |a: &Point, b: &Point| a.x * b.y - a.y * b.x;
            let dot = |a: &Point, b: &Point| a.x * b.x + a.y * b.y;
            // cba operator overloads :(
            let candidate = Point {
                x: p.x - self.start.x,
                y: p.y - self.start.y,
            };
            let line = Point {
                x: self.end.x - self.start.x,
                y: self.end.y - self.start.y,
            };
            if cross(&candidate, &line) == 0 {
                if dot(&candidate, &line) == 0 {
                    return true;
                } else if dot(&candidate, &line) == dot(&line, &line) {
                    return true;
                } else if dot(&candidate, &line) > 0 && dot(&line, &line) > dot(&candidate, &line) {
                    return true;
                }
            }
            return false;
        }
    }

    fn max(&self) -> Point {
        Point {
            x: cmp::max(self.start.x, self.end.x),
            y: cmp::max(self.start.y, self.end.y),
        }
    }
    fn min(&self) -> Point {
        Point {
            x: cmp::min(self.start.x, self.end.x),
            y: cmp::min(self.start.y, self.end.y),
        }
    }
}

fn parse_day5(input: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    for line in input.lines() {
        let mut line: Vec<Point> = line
            .trim()
            .split(" -> ")
            .map(|p| Point::from_str(p))
            .collect();
        if line.len() != 2 {
            continue;
        }
        line.sort();
        let start = line.remove(0);
        let end = line.remove(0);
        lines.push(Line { start, end });
    }
    lines
}

fn get_extent(lines: &Vec<Line>) -> (Point, Point) {
    let mut min = Point {
        x: i32::MAX,
        y: i32::MAX,
    };
    let mut max = Point { x: 0, y: 0 };
    for line in lines {
        min.x = cmp::min(line.min().x, min.x);
        min.y = cmp::min(line.min().y, min.y);
        max.x = cmp::max(line.max().x, max.x);
        max.y = cmp::max(line.max().y, max.y);
    }
    (min, max)
}

fn day5_solve(lines: &Vec<Line>, min: &Point, max: &Point) -> usize {
    let mut counts: HashMap<Point, usize> = HashMap::new();
    for y in min.y..max.y + 1 {
        for x in min.x..max.x + 1 {
            let candidate = Point { x, y };
            for line in lines {
                if line.intersect(&candidate) {
                    *counts.entry(candidate.clone()).or_insert(0) += 1;
                }
            }
        }
    }
    let positions = counts
        .iter()
        .filter(|p| p.1 >= &2)
        .map(|p| p.0)
        .collect::<Vec<&Point>>();
    positions.len()
}

fn day5_small_pt1() -> usize {
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    let lines = parse_day5(input);
    let (min, max) = get_extent(&lines);
    let lines: Vec<Line> = lines
        .into_iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        .collect();
    day5_solve(&lines, &min, &max)
}

fn day5_small_pt2() -> usize {
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    let lines = parse_day5(input);
    let (min, max) = get_extent(&lines);
    day5_solve(&lines, &min, &max)
}

fn day5_pt1() -> usize {
    let input = fs::read_to_string("input_day5.txt").expect("Can't read");
    let lines = parse_day5(&input);
    let lines: Vec<Line> = lines
        .into_iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        .collect();
    let (min, max) = get_extent(&lines);
    day5_solve(&lines, &min, &max)
}

fn day5_pt2() -> usize {
    let input = fs::read_to_string("input_day5.txt").expect("Can't read");
    let lines = parse_day5(&input);
    let (min, max) = get_extent(&lines);
    day5_solve(&lines, &min, &max)
}

fn main() {
    //println!("Day1 problem {}", day1());
    //println!("Day 2 problem {}", day1_pt2());
    //println!("{}", day2_pt1());
    //println!("{}", day2_pt2());
    //println!("pt1: {}", day3_pt1());
    //println!("pt2: {}", day3_pt2());
    //println!("pt1: {}", day4_pt1());
    //println!("pt2: {}", day4_pt2());
    //println!("pt1: {}", day5_pt1());
    println!("pt1: {}", day5_pt1());
    println!("pt2: {}", day5_pt2());
}
