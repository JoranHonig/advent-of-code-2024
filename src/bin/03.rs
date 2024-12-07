advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mul_instruction_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let multiplication_sum =
        mul_instruction_regex
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [a, b])| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .map(|(a, b)| a * b)
            .sum();


    Some(multiplication_sum)
}

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_instruction_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_instruction_regex = Regex::new(r"do").unwrap();
    let dont_instruction_regex = Regex::new(r"don't\(\)").unwrap();

    let do_instructions: Vec<(usize, Instruction)> = do_instruction_regex
        .find_iter(input)
        .map(|m| (m.start(), Instruction::Do))
        .collect::<Vec<_>>();

    let dont_instructions: Vec<(usize, Instruction)> = dont_instruction_regex
        .find_iter(input)
        .map(|m| (m.start(), Instruction::Dont))
        .collect::<Vec<_>>();

    let mul_instructions: Vec<(usize, Instruction)> = mul_instruction_regex
        .captures_iter(input)
        .map(|c| (c.get(0).unwrap().start(), c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()))
        .map(|(index, a, b)| (index, Instruction::Mul(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())))
        .collect::<Vec<_>>();

    let mut instructions = do_instructions
        .iter()
        .chain(dont_instructions.iter())
        .chain(mul_instructions.iter())
        .collect::<Vec<_>>();

    instructions.sort_by_key(|(index, _)| *index);

    // recompute result using reducer
    let result = instructions
        .iter()
        .fold((0, 1), |acc, (_, instruction)| {
            match instruction {
                Instruction::Mul(a, b) => (acc.0 + a * b * acc.1, acc.1),
                Instruction::Do => (acc.0, 1),
                Instruction::Dont => (acc.0, 0),
            }
        });

    Some(result.0)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
