use itertools::Itertools;

advent_of_code::solution!(7);
use rayon::prelude::*;

fn potential_results(numbers: &Vec<u64>, value: Option<u64>, goal: u64, with_concat: bool) -> Option<Vec<u64>> {
    // if we start out with an empty list
    if numbers.len() == 0 {
        return Some(vec![value?]);
    }

    let mut num_copy = numbers.clone();
    let head = num_copy.remove(0);

    // if we're dealing with the first number
    if value.is_none() {
        return potential_results(&num_copy, Some(head), goal, with_concat)
    }

    if value? > goal {
        return None
    }

    let mut result = vec![];

    if with_concat {
        let concat_value = format!("{}{}", value?, head).parse::<u64>().ok()?;
        if let Some(mut concat_result) = potential_results(&num_copy, Some(concat_value), goal, with_concat) {
            result.append(&mut concat_result);
        }
    }

    let mul_value = value? * head;
    if let Some(mut mul_result) = potential_results(&num_copy, Some(mul_value), goal, with_concat) {
        result.append(&mut mul_result);
    }

    let add_value = value? + head;
    if let Some(mut add_result) = potential_results(&num_copy, Some(add_value), goal, with_concat) {
        result.append(&mut add_result);
    }

    Some(result)
}

fn try_parse(input: &str) -> Result<Vec<(u64, Vec<u64>)>, ()> {
    input
        .lines()
        .map(|line| {
            let s = line.split(':').collect::<Vec<&str>>();
            let result = s.get(0).ok_or(())?.parse::<u64>().map_err(|_| ())?;
            let numbers = s.get(1).ok_or(())?.trim().split(' ').map(|n| n.parse::<u64>()).collect::<Result<Vec<u64>, _>>().map_err(|_| ())?;

            Ok((result, numbers))
        }).collect::<Result<Vec<(u64, Vec<u64>)>, ()>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = try_parse(input).ok()?;

    let result: u64 = parsed
        .par_iter()
        .map(|(result, numbers)| if potential_results(numbers, None, *result, false).unwrap_or(vec![]).contains(result) { *result } else { 0 })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = try_parse(input).ok()?;

    let result: u64 = parsed
        .par_iter()
        .map(|(result, numbers)| if potential_results(numbers, None, *result, true).unwrap_or(vec![]).contains(result) { *result } else { 0 })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
