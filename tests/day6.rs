struct Fish {
    _counter: u8,
}

impl Fish {
    fn from_input(input: &str) -> Vec<Fish> {
        input
            .split(',')
            .map(|i| Fish {
                _counter: i.parse::<u8>().unwrap(),
            })
            .collect()
    }

    fn update(&mut self) -> Option<Fish> {
        match self._counter {
            0 => {
                self._counter = 6;
                Some(Fish { _counter: 8 })
            }
            _ => {
                self._counter -= 1;
                None
            }
        }
    }
}

fn process(initial_state: &mut Vec<Fish>, epochs: usize) -> usize {
    for _ in 0..epochs {
        let new_spawns: Vec<Fish> = initial_state
            .iter_mut()
            .map(|f| f.update())
            .filter(|f| match f {
                Some(_) => true,
                None => false,
            })
            .flatten()
            .collect();
        initial_state.extend(new_spawns.into_iter());
    }
    initial_state.len()
}

// use non naive implementation
fn process_sane(input: &str, epochs: usize) -> usize {
    const NUM_STATES: usize = 9;
    let mut current: [usize; NUM_STATES] = [0; NUM_STATES];
    for i in input.split(",") {
        current[i.parse::<usize>().unwrap()] += 1;
    }
    for _ in 0..epochs {
        let mut next = [0; NUM_STATES];
        for i in (0..NUM_STATES).rev() {
            match i {
                0 => {
                    next[6] += current[i];
                    next[8] = current[i];
                }
                _ => next[i - 1] = current[i],
            }
        }
        current = next;
    }
    current.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "3,4,3,1,2";
    const FULL_INPUT: &str = "2,5,3,4,4,5,3,2,3,3,2,2,4,2,5,4,1,1,4,4,5,1,2,1,5,2,1,5,1,1,1,2,4,3,3,1,4,2,3,4,5,1,2,5,1,2,2,5,2,4,4,1,4,5,4,2,1,5,5,3,2,1,3,2,1,4,2,5,5,5,2,3,3,5,1,1,5,3,4,2,1,4,4,5,4,5,3,1,4,5,1,5,3,5,4,4,4,1,4,2,2,2,5,4,3,1,4,4,3,4,2,1,1,5,3,3,2,5,3,1,2,2,4,1,4,1,5,1,1,2,5,2,2,5,2,4,4,3,4,1,3,3,5,4,5,4,5,5,5,5,5,4,4,5,3,4,3,3,1,1,5,2,4,5,5,1,5,2,4,5,4,2,4,4,4,2,2,2,2,2,3,5,3,1,1,2,1,1,5,1,4,3,4,2,5,3,4,4,3,5,5,5,4,1,3,4,4,2,2,1,4,1,2,1,2,1,5,5,3,4,1,3,2,1,4,5,1,5,5,1,2,3,4,2,1,4,1,4,2,3,3,2,4,1,4,1,4,4,1,5,3,1,5,2,1,1,2,3,3,2,4,1,2,1,5,1,1,2,1,2,1,2,4,5,3,5,5,1,3,4,1,1,3,3,2,2,4,3,1,1,2,4,1,1,1,5,4,2,4,3";

    #[test]
    fn part1_small() {
        let mut state = Fish::from_input(SMALL_INPUT);
        assert_eq!(5934, process(&mut state, 80));
    }
    #[test]
    fn part1() {
        let mut state = Fish::from_input(FULL_INPUT);
        assert_eq!(345387, process(&mut state, 80));
    }
    #[test]
    fn part2_small() {
        assert_eq!(26984457539, process_sane(SMALL_INPUT, 256));
    }
    #[test]
    fn part2() {
        assert_eq!(1574445493136, process_sane(FULL_INPUT, 256));
    }
}
