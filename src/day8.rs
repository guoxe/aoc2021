enum Segments {
    One = 2,
    Four = 4,
    Seven = 3,
    Eight = 7,
}

fn count_output_digits_unique_segments(input: &str) -> usize {
    let unique_digit = |len| match len {
        2 => true,
        3 => true,
        4 => true,
        7 => true,
        _ => false,
    };
    let count_uniques_in_output = |o: &str| o.split(" ").filter(|d| unique_digit(d.len())).count();
    input
        .lines()
        .filter_map(|l| l.split(" | ").nth(1))
        .map(|o| count_uniques_in_output(o))
        .sum()
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
}
