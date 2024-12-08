use itertools::Itertools;
use std::collections::{LinkedList, HashMap, HashSet};

advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy)]
struct Rule {
    first: u32,
    second: u32,
}

fn try_parse<'a>(input: &'a str) -> Result<(Vec<Rule>, Vec<Vec<u32>>), ()> {
    let mut split_input = input.split("\n\n");

    let rules = split_input.next().ok_or(())?;

    let parsed_rules = rules.lines().map(|line| {
        let mut values = line.split('|');
        let first = values.next().ok_or(())?.parse::<u32>().map_err(|_| ())?;
        let second = values.next().ok_or(())?.parse::<u32>().map_err(|_| ())?;

        Ok(Rule { first, second })
    }).collect::<Result<Vec<Rule>,()>>()?;

    let updates = split_input.next().ok_or(())?;

    let parsed_updates = updates
        .lines()
        .map(|line| line
                    .split(',')
                    .map(|value| value.parse::<u32>().map_err(|_| ()))
                    .collect::<Result<Vec<u32>,()>>()
        ).collect::<Result<Vec<Vec<u32>>,()>>()?;

    Ok((parsed_rules, parsed_updates))
}

fn valid_update(update: &Vec<u32>, rule_map: &HashMap<u32, Vec<Rule>>) -> bool {
    let mut illegals = HashSet::new();
    let mut stack = update.clone();
    stack.reverse();

    while let Some(value) = stack.pop() {
        if illegals.contains(&value) {
            return false;
        }
        if let Some(rules) = rule_map.get(&value) {
            illegals.extend(rules.iter().map(|rule| rule.first));
        }
    }

    true
}

fn middle_element(update: &Vec<u32>) -> Option<u32> {
    update.get(update.len() / 2).copied()
}

fn unconstrained(number: &u32, rules: &Vec<Rule>) -> bool {
    rules.iter().all(|rule| rule.second != *number)
}

fn sort_update(update: &Vec<u32>, rules: Vec<Rule>) -> Option<Vec<u32>> {
    let mut result = vec![];
    let mut stack = update.clone();
    let mut adjusted_rules = rules;

    adjusted_rules.retain(|rule| stack.contains(&rule.second) && stack.contains(&rule.first));

    for _ in 0..stack.len() {
        let first_index = stack.iter().position(|number| unconstrained(number, &adjusted_rules));
        let item = stack.remove(first_index?);
        result.push(item);
        adjusted_rules.retain(|rule| rule.first != item);
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = try_parse(input).ok()?;

    let map_by_second = rules.into_iter().fold(
        std::collections::HashMap::new(),
        |mut acc, rule| {
            acc.entry(rule.second).or_insert(vec![]).push(rule);
            acc
        }
    );

    let answer = updates
        .iter()
        .filter(|update| valid_update(update, &map_by_second))
        .map(middle_element)
        .collect::<Option<Vec<u32>>>()?
        .iter()
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = try_parse(input).ok()?;

    let map_by_second = rules.clone().into_iter().fold(
        std::collections::HashMap::new(),
        |mut acc, rule| {
            acc.entry(rule.second).or_insert(vec![]).push(rule);
            acc
        }
    );

    let answer = updates
        .iter()
        .filter(|update| !valid_update(update, &map_by_second))
        .map(|update| sort_update(update, rules.clone()))
        .collect::<Option<Vec<Vec<u32>>>>()?
        .iter()
        .map(|update| middle_element(update))
        .collect::<Option<Vec<u32>>>()?
        .iter()
        .sum::<u32>();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
