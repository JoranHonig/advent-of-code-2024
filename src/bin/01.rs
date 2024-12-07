use std::collections::HashMap;

advent_of_code::solution!(01);


fn split_to_integers(input: &str) -> Option<(u32, u32)> {
    let mut digits = input.split_ascii_whitespace();
    let first = digits.next()?.parse::<u32>().ok()?;
    let second = digits.next()?.parse::<u32>().ok()?;
    Some((first, second))
}

fn pre_process(input: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let tuples = input
        .lines()
        .map(|tuple_string| split_to_integers(tuple_string))
        .collect::<Option<Vec<_>>>()?;

    let first_elements = tuples.iter().map(|(first, _)| *first).collect::<Vec<u32>>();
    let second_elements = tuples.iter().map(|(_, second)| *second).collect::<Vec<u32>>();

    Some((first_elements, second_elements))
}

fn count_occurences(input: Vec<u32>) -> HashMap<u32, u32> {
    let mut counts = HashMap::new();
    for element in input {
        *counts.entry(element).or_insert(0) += 1;
    }
    counts
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first_elements, mut second_elements) = pre_process(input)?;

    first_elements.sort();
    second_elements.sort();

    let differences = first_elements.iter().zip(second_elements.iter()).map(|(first, second)| first.abs_diff(*second));

    let total_difference = differences.sum();

    Some(total_difference)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_elements, second_elements) = pre_process(input)?;

    let counts = count_occurences(second_elements);

    let distance: u32 = first_elements.iter().map(|number| *number * counts.get(number).unwrap_or(&0)).sum();

    Some(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
