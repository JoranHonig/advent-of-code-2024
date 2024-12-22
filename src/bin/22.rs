use itertools::Itertools;
use std::collections::HashMap;
advent_of_code::solution!(22);

#[derive(Debug, Copy, Clone)]
struct SecretNumber {
    number: u64,
}

impl SecretNumber {
    fn prune(&mut self) {
        self.number = self.number % 16777216;
    }

    fn mix(&mut self, other: u64) {
        self.number = self.number ^ other;
    }

    fn next(&mut self) {
        self.mix(self.number * 64);
        self.prune();

        self.mix(self.number / 32);
        self.prune();

        self.mix(self.number * 2048);
        self.prune();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| {
            let mut secret_number = SecretNumber {
                number: line.parse().unwrap(),
            };

            (0..2000).for_each(|_| secret_number.next());

            Some(secret_number.number)
        })
        .sum()
}

fn triggers(start_number: u64) -> HashMap<(i128, i128, i128, i128), i128> {
    let mut secret_number = SecretNumber {
        number: start_number,
    };
    let prices =
        (0..2000).map(|_| {
        let result = secret_number.number % 10;
        secret_number.next();
        result
            })
        .tuple_windows()
        .map(|(a, b)| (b as i128 - a as i128, b)).collect_vec();

    // println!("{:?}", prices);

    let mut price_triggers: HashMap<(i128, i128, i128, i128), i128> = HashMap::new();

    prices.into_iter().tuple_windows::<(_, _, _, _)>().for_each(|((a, _), (b, _), (c, _), (d, v))| {
        price_triggers.entry((a, b, c, d)).or_insert(v as i128);
    });

    price_triggers
}

pub fn part_two(input: &str) -> Option<u32> {
    let trigger_maps: Vec<HashMap<(i128, i128, i128, i128), i128>> =
        input
        .lines()
        .map(|line| {
            triggers(line.parse().unwrap())
        }).collect();

    let mut result = HashMap::new();

    for map in trigger_maps {
        for (key, &value) in map.iter() {
            *result.entry(key.clone()).or_insert(0) += value;
        }
    }


    result.iter().max_by_key(|(_, &value)| value).map(|(_key, &value)| {
        value as u32})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
