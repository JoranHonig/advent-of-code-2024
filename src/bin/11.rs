advent_of_code::solution!(11);
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Stone {
    NormalStone(u64),
}

fn efficient_blink_mutation(stone: &Stone) -> Vec<Stone> {
    match stone {
        Stone::NormalStone(number) => {
            if *number == 0 {
                return vec![Stone::NormalStone(1)];
            }

            let string_number = number.to_string();

            if string_number.len() % 2 == 0 {
                let first = string_number[0..string_number.len() / 2].parse::<u64>().unwrap();
                let second = string_number[string_number.len() / 2..].parse::<u64>().unwrap();
                return vec![Stone::NormalStone(first), Stone::NormalStone(second)];
            } else {
                return vec![Stone::NormalStone(number * 2024)];
            }
        },
    }
}


fn memoized_blink_stone(stone: &Stone, memory: &mut HashMap<(Stone, u64), u64>, blinks: u64) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(result) = memory.get(&(stone.clone(), blinks)) {
        *result
    } else {
        let blink_result = efficient_blink_mutation(&stone);
        let result = blink_result.iter().map(|x| memoized_blink_stone(x, memory, blinks - 1)).sum();
        memory.insert((stone.clone(), blinks), result);
        result
    }
}


fn blink_mutation(number: u64) -> Vec<u64> {
    if number == 0 {
        return vec![1];
    }

    let string_number = number.to_string();

    if string_number.len() % 2 == 0 {
        let first = string_number[0..string_number.len() / 2].parse::<u64>().unwrap();
        let second = string_number[string_number.len() / 2..].parse::<u64>().unwrap();
        return vec![first, second];
    } else {
        return vec![number * 2024];
    }
}

fn blink(numbers: Vec<u64>) -> Vec<u64> {
    numbers.par_iter().flat_map(|&x| blink_mutation(x)).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut current = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut previous;

    for _ in 0..25 {
        previous = current;
        current = blink(previous);
    }


    Some(current.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let current = input
        .trim()
        .split_whitespace()
        .map(|x| Stone::NormalStone(x.parse::<u64>().unwrap()))
        .collect::<Vec<Stone>>();

    let mut memory = HashMap::new();
    let result = current.iter().map(|x| memoized_blink_stone(x, &mut memory, 75)).sum::<u64>();

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
