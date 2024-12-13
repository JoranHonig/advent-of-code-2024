
use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct ClawMachine {
    prize: (u64, u64),
    button_a: (u64, u64),
    button_b: (u64, u64),
}

impl ClawMachine {
    fn try_solve(&self, allow_hundred: bool) -> Option<(u64, u64)> {
        let mut formula_0 = (self.button_a.0, self.button_b.0, self.prize.0);
        let mut formula_1 = (self.button_a.1, self.button_b.1, self.prize.1);

        let mut multiplier = self.button_a.1;
        formula_0 = (multiplier * formula_0.0, multiplier * formula_0.1, multiplier* formula_0.2);

        multiplier = self.button_a.0;
        formula_1 = (multiplier * formula_1.0, multiplier * formula_1.1, multiplier* formula_1.2);

        let result;
        let y;

        if formula_0.2 > formula_1.2  && formula_0.1 > formula_1.1 {
            result = formula_0.2 - formula_1.2;
            y = formula_0.1 - formula_1.1;
        } else if formula_0.2 < formula_1.2 && formula_0.1 < formula_1.1 {
            result = formula_1.2 - formula_0.2;
            y = formula_1.1 - formula_0.1;
        } else {
            return None;
        }

        let y_value = result / y;

        if self.prize.0 < self.button_b.0 * y_value {
            return None;
        }

        let x_value = (self.prize.0 - self.button_b.0 * y_value) / self.button_a.0;

        if !allow_hundred && (x_value > 100 || y_value > 100) {
            return None;
        }

        if x_value * self.button_a.0 + y_value * self.button_b.0 != self.prize.0 {
            None
        } else if x_value * self.button_a.1 + y_value * self.button_b.1 != self.prize.1 {
            None
        } else {
            Some((x_value, y_value))
        }
    }
}

impl TryFrom<&str> for ClawMachine {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let button_a_str = lines.next().ok_or(())?;
        let button_b_str = lines.next().ok_or(())?;
        let prize_str = lines.next().ok_or(())?;

        // use regex to capture x and y values "Button A: X+94, Y+34"
        let pattern = Regex::new(r".*X.(\d+), Y.(\d+)").unwrap();
        let button_a = pattern
            .captures(button_a_str)
            .ok_or(())?
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let button_b = pattern
            .captures(button_b_str)
            .ok_or(())?
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let prize = pattern
            .captures(prize_str)
            .ok_or(())?
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            prize: (prize.get(0).ok_or(())?.to_owned(), prize.get(1).ok_or(())?.to_owned()),
            button_a: (button_a.get(0).ok_or(())?.to_owned(), button_a.get(1).ok_or(())?.to_owned()),
            button_b: (button_b.get(0).ok_or(())?.to_owned(), button_b.get(1).ok_or(())?.to_owned()),
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let claw_machines = input
        .split("\n\n")
        .map(|x| ClawMachine::try_from(x))
        .collect::<Result<Vec<ClawMachine>, ()>>().ok()?;

    let results =
        claw_machines
            .iter()
            .filter_map(|m| m.try_solve(false))
            .collect_vec();

    let cost =
        results
            .iter()
            .map(|(x, y)| x * 3 + y)
            .sum::<u64>();

    Some(cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let claw_machines = input
        .split("\n\n")
        .map(|x| ClawMachine::try_from(x))
        .collect::<Result<Vec<ClawMachine>, ()>>().ok()?;

    let results =
        claw_machines
            .iter()
            .map(|m| ClawMachine {
                button_a: (m.button_a.1, m.button_a.0),
                button_b: (m.button_b.1, m.button_b.0),
                prize: (m.prize.1 + 10000000000000, m.prize.0 + 10000000000000),
            })
            .filter_map(|m| m.try_solve(true))
            .collect_vec();

    let cost =
        results
            .iter()
            .map(|(x, y)| x * 3 + y)
            .sum::<u64>();

    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
