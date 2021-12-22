use std::collections::{HashMap, HashSet};

fn count_output_digits_unique_segments(input: &str) -> usize {
    let unique_digit = |len| matches!(len, 2 | 3 | 4 | 7);
    let count_uniques_in_output = |o: &str| o.split(' ').filter(|d| unique_digit(d.len())).count();
    input
        .lines()
        .filter_map(|l| l.split(" | ").nth(1))
        .map(count_uniques_in_output)
        .sum()
}

fn num_shared_chars(one: &str, two: &str) -> usize {
    let probe: HashSet<char> = one.chars().collect();
    two.chars().filter(|c| probe.contains(c)).count()
}

fn calculate_score(codemap: &HashMap<u32, String>, output: &str) -> u32 {
    let mut assembled = String::new();
    for signal in output.split(' ') {
        for (key, val) in codemap.iter() {
            if num_shared_chars(val, signal) == signal.len() && signal.len() == val.len() {
                assembled.push(char::from_digit(*key, 10).unwrap());
            }
        }
    }
    assembled.parse().unwrap()
}

fn decode_output(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut decoded: HashMap<u32, String> = HashMap::new();
        let mut parts = line.split(" | ");
        let mut initial: Vec<&str> = parts.next().unwrap().split(' ').collect();
        let mut remaining: Vec<&str> = Vec::new();
        let output = parts.next().unwrap();
        for s in initial.drain(..) {
            match s.len() {
                2 => {
                    decoded.insert(1, s.to_string());
                }
                3 => {
                    decoded.insert(7, s.to_string());
                }
                4 => {
                    decoded.insert(4, s.to_string());
                }
                7 => {
                    decoded.insert(8, s.to_string());
                }
                _ => {
                    remaining.push(s);
                }
            }
        }
        decoded.insert(
            2,
            remaining
                .iter()
                .filter(|c| num_shared_chars(c, decoded.get(&4).unwrap()) == 2)
                .filter(|c| num_shared_chars(c, decoded.get(&7).unwrap()) == 2)
                .filter(|c| num_shared_chars(c, decoded.get(&8).unwrap()) == 5)
                .next()
                .unwrap()
                .to_string(),
        );
        remaining.retain(|x| x != decoded.get(&2).unwrap());
        decoded.insert(
            3,
            remaining
                .iter()
                .filter(|c| num_shared_chars(c, decoded.get(&8).unwrap()) == 5)
                .filter(|c| num_shared_chars(c, decoded.get(&2).unwrap()) == 4)
                .next()
                .unwrap()
                .to_string(),
        );
        remaining.retain(|x| x != decoded.get(&3).unwrap());
        decoded.insert(
            6,
            remaining
                .iter()
                .filter(|c| num_shared_chars(c, decoded.get(&1).unwrap()) == 1)
                .filter(|c| num_shared_chars(c, decoded.get(&2).unwrap()) == 4)
                .next()
                .unwrap()
                .to_string(),
        );
        remaining.retain(|x| x != decoded.get(&6).unwrap());
        decoded.insert(
            9,
            remaining
                .iter()
                .filter(|c| num_shared_chars(c, decoded.get(&8).unwrap()) == 6)
                .filter(|c| num_shared_chars(c, decoded.get(&4).unwrap()) == 4)
                .next()
                .unwrap()
                .to_string(),
        );
        remaining.retain(|x| x != decoded.get(&9).unwrap());
        decoded.insert(
            0,
            remaining
                .iter()
                .filter(|c| num_shared_chars(c, decoded.get(&8).unwrap()) == 6)
                .next()
                .unwrap()
                .to_string(),
        );
        remaining.retain(|x| x != decoded.get(&0).unwrap());
        decoded.insert(5, remaining.pop().unwrap().to_string());
        let score = calculate_score(&decoded, output);
        total += score;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn day8_part1_small() {
        let input = read_input_small("day8");
        assert_eq!(26, count_output_digits_unique_segments(&input));
    }

    #[test]
    fn day8_part1_full() {
        let input = read_input_full("day8");
        assert_eq!(421, count_output_digits_unique_segments(&input));
    }

    #[test]
    fn day8_part2_tiny() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(5353, decode_output(&input));
    }

    #[test]
    fn day8_part2_small() {
        let input = read_input_small("day8");
        assert_eq!(61229, decode_output(&input));
    }

    #[test]
    fn day8_part2_full() {
        let input = read_input_full("day8");
        assert_eq!(986163, decode_output(&input));
    }
}
