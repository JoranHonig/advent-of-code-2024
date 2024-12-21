advent_of_code::solution!(21);
use std::collections::HashMap;
use cached::proc_macro::cached;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum NumberPadKey {
    None,
    Number(u128),
    Select
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum DirectionalPadKey {
    None,
    Up,
    Down,
    Left,
    Right,
    Select
}

impl TryFrom<&char> for NumberPadKey {
    type Error = ();

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        if value.is_digit(10) {
            return Ok(NumberPadKey::Number(value.to_digit(10).unwrap() as u128));
        }

        if value == &'A' {
            return Ok(NumberPadKey::Select);
        }

        Err(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Select
}

impl Into<DirectionalPadKey> for Direction {
    fn into(self) -> DirectionalPadKey {
        match self {
            Direction::Up => DirectionalPadKey::Up,
            Direction::Down => DirectionalPadKey::Down,
            Direction::Left => DirectionalPadKey::Left,
            Direction::Right => DirectionalPadKey::Right,
            Direction::Select => DirectionalPadKey::Select,
        }
    }
}

const NUMBERPAD: [[NumberPadKey; 3]; 4] = [
    [NumberPadKey::Number(7), NumberPadKey::Number(8), NumberPadKey::Number(9)],
    [NumberPadKey::Number(4), NumberPadKey::Number(5), NumberPadKey::Number(6)],
    [NumberPadKey::Number(1), NumberPadKey::Number(2), NumberPadKey::Number(3)],
    [NumberPadKey::None, NumberPadKey::Number(0), NumberPadKey::Select],
];

const DIRECTIONALPAD: [[DirectionalPadKey; 3]; 2] = [
    [DirectionalPadKey::None, DirectionalPadKey::Up, DirectionalPadKey::Select],
    [DirectionalPadKey::Left, DirectionalPadKey::Down, DirectionalPadKey::Right],
];

trait InputPad<T> {
    fn press_key(&mut self, key: T) -> Result<(), ()>;
    fn sequence(&self) -> Vec<Direction>;
}

struct NumberPad {
    cursor: (usize, usize),
    sequence: Vec<Direction>,
    indices: HashMap<NumberPadKey, (usize, usize)>,
}

impl InputPad<NumberPadKey> for NumberPad {
    fn press_key(&mut self, key: NumberPadKey) -> Result<(), ()>{
        let (key_x, key_y) = *self.indices.get(&key).ok_or(())?;
        let (cursor_x, cursor_y) = self.cursor;

        let x_diff = key_x as isize - cursor_x as isize;
        let y_diff = key_y as isize - cursor_y as isize;

        let y_direction = if y_diff > 0 { Direction::Down } else { Direction::Up };
        let x_direction = if x_diff > 0 { Direction::Right } else { Direction::Left };

        let x_first_illegal = self.cursor.1 == 3 && key_x == 0;
        let y_first_illegal = self.cursor.0 == 0 && key_y == 3;


        if x_diff < 0 && !x_first_illegal {
            self.sequence.extend((0..x_diff.abs()).map(|_| x_direction));
            self.sequence.extend((0..y_diff.abs()).map(|_| y_direction));
        } else if y_diff > 0 && !y_first_illegal {
            self.sequence.extend((0..y_diff.abs()).map(|_| y_direction));
            self.sequence.extend((0..x_diff.abs()).map(|_| x_direction));
        } else if x_first_illegal {
            self.sequence.extend((0..y_diff.abs()).map(|_| y_direction));
            self.sequence.extend((0..x_diff.abs()).map(|_| x_direction));
        } else {
            self.sequence.extend((0..x_diff.abs()).map(|_| x_direction));
            self.sequence.extend((0..y_diff.abs()).map(|_| y_direction));
        }

        self.sequence.push(Direction::Select);

        self.cursor = (key_x, key_y);
        Ok(())
    }

    fn sequence(&self) -> Vec<Direction> {
        self.sequence.clone()
    }
}

struct DirectionalPad {
    sequence: Vec<Direction>,
    cursor: (usize, usize),
    indices: HashMap<DirectionalPadKey, (usize, usize)>,
}

impl InputPad<DirectionalPadKey> for DirectionalPad {
    fn press_key(&mut self, key: DirectionalPadKey) -> Result<(), ()>{
        let (key_x, key_y) = *self.indices.get(&key).ok_or(())?;
        let (cursor_x, cursor_y) = self.cursor;

        let x_diff = key_x as isize - cursor_x as isize;
        let y_diff = key_y as isize - cursor_y as isize;

        let y_direction = if y_diff > 0 { Direction::Down } else { Direction::Up };
        let x_direction = if x_diff > 0 { Direction::Right } else { Direction::Left };

        let x_first_illegal = self.cursor.1 == 0 && key_x == 0;
        let y_first_illegal = self.cursor.0 == 0 && key_y == 0;

        if x_diff < 0 && !x_first_illegal {
            self.sequence.extend((0..x_diff.abs()).map(|_| x_direction));
            self.sequence.extend((0..y_diff.abs()).map(|_| y_direction));
        } else if y_diff < 0 && !y_first_illegal {
            self.sequence.extend((0..y_diff.abs()).map(|_| y_direction));
            self.sequence.extend((0..x_diff.abs()).map(|_| x_direction));
        } else if y_first_illegal{
            self.sequence.extend((0..x_diff.abs()).map(|_| x_direction));
            self.sequence.extend((0..y_diff.abs()).map(|_| y_direction));
        } else {
            self.sequence.extend((0..y_diff.abs()).map(|_| y_direction));
            self.sequence.extend((0..x_diff.abs()).map(|_| x_direction));
        }


        self.sequence.push(Direction::Select);

        self.cursor = (key_x, key_y);
        Ok(())
    }

    fn sequence(&self) -> Vec<Direction> {
        self.sequence.clone()
    }
}

impl DirectionalPad {
    fn new() -> Self {
        Self {
            sequence: Vec::new(),
            cursor: (2, 0),
            indices: DIRECTIONALPAD.iter().enumerate().flat_map(|(y, row)| {
                row.iter().enumerate().map(move |(x, key)| {
                    (*key, (x, y))
                })
            }).collect(),
        }
    }
}

impl NumberPad {
    fn new() -> Self {
        Self {
            cursor: (2, 3),
            sequence: Vec::new(),
            indices: NUMBERPAD.iter().enumerate().flat_map(|(y, row)| {
                row.iter().enumerate().map(move |(x, key)| {
                    (*key, (x, y))
                })
            }).collect(),
        }
    }
}


pub fn part_one(input: &str) -> Option<u128> {
    let puzzles = input
        .lines()
        .map(|line| line.chars().map(|c| NumberPadKey::try_from(&c)).collect::<Result<Vec<NumberPadKey>, ()>>())
        .collect::<Result<Vec<Vec<NumberPadKey>>, ()>>().unwrap();

    let mut result = 0;

    for puzzle in puzzles {
        let mut number_pad = NumberPad::new();
        let mut robot_1 = DirectionalPad::new();
        let mut robot_2 = DirectionalPad::new();

        puzzle.iter().for_each(|key| number_pad.press_key(*key).unwrap());
        number_pad.sequence().iter().for_each(|key| robot_1.press_key((*key).into()).unwrap());
        robot_1.sequence().iter().for_each(|key| robot_2.press_key((*key).into()).unwrap());

        let number = puzzle.iter().filter_map(|key| {
            match key {
                NumberPadKey::Number(n) => Some(n),
                _ => None
            }
        }).fold(0, |acc, n| acc*10 + n);


        result += number * robot_2.sequence().len() as u128;
    }


    Some(result)
}

#[cached]
fn efficient_solve_part(moves: Vec<Direction>, depth: u128) -> u128 {
    let mut robot = DirectionalPad::new();
    moves.iter().for_each(|key| robot.press_key((*key).into()).unwrap());
    return efficient_solve(robot.sequence(), depth - 1);
}

fn efficient_solve(moves: Vec<Direction>, depth: u128) -> u128 {
    if depth == 0 {
        return moves.len() as u128;
    }

    let mut parts = moves.split(|f| *f == Direction::Select).map(|m| {
        let mut moves = m.to_vec();
        moves.push(Direction::Select);
        moves
    }).collect::<Vec<Vec<Direction>>>();

    parts.pop();

    parts.into_iter().map(|m| efficient_solve_part(m, depth)).sum()
}

pub fn part_two(input: &str) -> Option<u128> {

    let puzzles = input
        .lines()
        .map(|line| line.chars().map(|c| NumberPadKey::try_from(&c)).collect::<Result<Vec<NumberPadKey>, ()>>())
        .collect::<Result<Vec<Vec<NumberPadKey>>, ()>>().unwrap();

    let mut result = 0;

    for puzzle in puzzles {
        let mut number_pad = NumberPad::new();
        puzzle.iter().for_each(|key| number_pad.press_key(*key).unwrap());

        let sequence_length = efficient_solve(number_pad.sequence(), 25);

        let number = puzzle.iter().filter_map(|key| {
            match key {
                NumberPadKey::Number(n) => Some(n),
                _ => None
            }
        }).fold(0, |acc, n| acc*10 + n);


        result += number * sequence_length;
    }


    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154154076501218));
    }
}
