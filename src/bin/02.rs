advent_of_code::solution!(2);

fn pre_process(input: &str) -> Option<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line|
            line
                .split_ascii_whitespace()
                .map(|number_string| number_string.parse::<u32>().ok())
                .collect::<Option<Vec<u32>>>()
        )
        .collect::<Option<Vec<Vec<u32>>>>()
}

fn is_valid(items: &Vec<u32>) -> bool {
    let differences: Vec<i32> = items.iter().map(|v| *v as i32).collect::<Vec<i32>>().windows(2).map(|w| w[1] - w[0]).collect();
    test_diffs(&differences)
}

fn is_mistake(is_ascending: bool, value: i32) -> bool {
    (is_ascending && value < 0) || (!is_ascending && value > 0) || value.abs() > 3 || value.abs() < 1
}

fn test_diffs(diffs: &Vec<i32>) -> bool {
    let ascending_amount = diffs.iter().filter(|n| **n >= 0).count();
    let is_ascending = ascending_amount > diffs.iter().count() - ascending_amount;

    diffs.iter().all(|value| !is_mistake(is_ascending, *value))
}

fn damped(items: &Vec<u32>) -> bool {
    // gather input state
    let mut differences: Vec<i32> = items.iter().map(|v| *v as i32).collect::<Vec<i32>>().windows(2).map(|w| w[1] - w[0]).collect();
    let ascending_amount = differences.iter().filter(|n| **n >= 0).count();
    let is_ascending = ascending_amount > differences.iter().count() - ascending_amount;

    // find the first mistake
    let mistakes = differences.iter().map(|value| is_mistake(is_ascending, *value)).collect::<Vec<bool>>();
    let mistake_index_option = mistakes.iter().position(|m| *m);
    if mistake_index_option.is_none() {
        return true;
    }
    let mistake_index = mistake_index_option.unwrap();

    // remove the first mistake
    let mistake = differences.remove(mistake_index);

    // add the difference from the mistake before & after
    let mut before = differences.clone();
    let mut after = differences.clone();

    if mistake_index == 0 {
        after[0] += mistake;
    } else if mistake_index == differences.len() {
        before[mistake_index - 1] += mistake;
    } else {
        before[mistake_index - 1] += mistake;
        after[mistake_index] += mistake;
    }

    // valid if either version is valid
    test_diffs(&before) || test_diffs(&after)
}

pub fn part_one(input: &str) -> Option<u32> {
    let levels = pre_process(input)?;

    let safe_count = levels
        .iter()
        .map(is_valid)
        .filter(|valid| *valid)
        .count();

    Some(safe_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let levels = pre_process(input)?;

    let safe_count = levels.clone()
        .iter()
        .map(damped)
        .filter(|valid| *valid)
        .count();

    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
