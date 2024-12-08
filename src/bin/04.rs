advent_of_code::solution!(4);

use itertools::{iproduct, Itertools};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Letter {
    X,
    M,
    A,
    S
}

impl Letter {
    fn next_letter(&self) -> Option<Letter> {
        match self {
            Letter::X => Some(Letter::M),
            Letter::M => Some(Letter::A),
            Letter::A => Some(Letter::S),
            Letter::S => None
        }
    }
}

impl TryFrom<&str> for Letter {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "X" => Ok(Letter::X),
            "M" => Ok(Letter::M),
            "A" => Ok(Letter::A),
            "S" => Ok(Letter::S),
            _ => Err(())
        }
    }
}

trait Grid<T> {
    fn get(&self, row: usize, column: usize) -> Option<&T>;
    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T>;

    fn dimensions(&self) -> (usize, usize);
}

struct LetterGrid {
    data: Vec<Vec<Letter>>,
    rows: usize,
    columns: usize,
}

impl Grid<Letter> for LetterGrid {
    fn get(&self, row: usize, column: usize) -> Option<&Letter> {
        self.data.get(row)?.get(column)
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Letter> {
        self.data.get_mut(row)?.get_mut(column)
    }

    fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

impl LetterGrid {
    fn new(data: Vec<Vec<Letter>>) -> Self {
        let rows = data.len();
        let columns = data.get(0).map(|row| row.len()).unwrap_or(0);

        LetterGrid {
            data ,
            rows,
            columns
        }
    }

    fn items(&mut self) -> Vec<&mut Letter> {
        self.data.iter_mut().flat_map(|row| row.iter_mut()).collect()
    }
}

impl TryFrom<&str> for LetterGrid {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let matrix = input.lines().enumerate().map(|(_, line)| {
            line.chars().enumerate().map(|(_, character)|
                Letter::try_from(character.to_string().as_str())
            ).collect::<Result<Vec<Letter>, ()>>()
        }).collect::<Result<Vec<Vec<Letter>>, ()>>()?;

        Ok(LetterGrid::new(matrix))
    }
}

fn indices_in_direction(index: &(usize, usize), direction: &(i32, i32), number: i32) -> Vec<(i32, i32)> {
    (0..=number).map(|i| ((index.0) as i32 + i * direction.0, (index.1) as i32 + i * direction.1)).collect()
}

fn valid_index<T>(index: &(i32, i32), grid: &dyn Grid<T>) -> bool {
    index.0 >= 0 && index.1 >= 0 && index.0 < grid.dimensions().0 as i32 && index.1 < grid.dimensions().1 as i32
}

fn to_letter<'a> (index: &(i32, i32), grid: &'a dyn Grid<Letter>) -> Option<Letter> {
    grid.get(index.0 as usize, index.1 as usize).copied()
}

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIAG_OFFSETS: [(i32, i32); 4] = [
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

const XMAS: [Letter; 4] = [
    Letter::X,
    Letter::M,
    Letter::A,
    Letter::S,
];


pub fn part_one(input: &str) -> Option<u32> {
    let grid = LetterGrid::try_from(input).ok()?;

    let (x, y) = grid.dimensions();
    let indices = iproduct!(0..x, 0..y);

    let possibilities = indices
        .flat_map(move |index| {
            NEIGHBOR_OFFSETS
                .iter()
                .map(move |&direction| indices_in_direction(&index, &direction, 3))
                .filter(|word| word.iter().all(|i| valid_index(i, &grid)))
                .map(|word| word.iter().map(|i| to_letter(i, &grid)).collect::<Option<Vec<Letter>>>())
                .filter(|word: &Option<Vec<Letter>>| word.as_ref().is_some_and(|w| w.as_slice() == XMAS))
                .collect::<Vec<_>>()
        })
        .count();

    Some(possibilities as u32)
}

fn diagonals_ma(word: &Vec<Letter>) -> bool {
    if word.len() != 4 {
        return false;
    }

    if word.contains(&Letter::A) || word.contains(&Letter::X) {
        return false;
    }

    if word[0] == word[3] || word[1] == word[2] {
        return false;
    }

    return true;
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = LetterGrid::try_from(input).ok()?;

    let (x, y) = grid.dimensions();
    let indices = iproduct!(0..x, 0..y);

    let answer =
        indices
            .filter(|index| to_letter(&(index.0 as i32, index.1 as i32), &grid) == Some(Letter::A))
            .map(move |index| {
                DIAG_OFFSETS
                    .iter()
                    .map(move |&direction| indices_in_direction(&index, &direction, 1).remove(1))
                    .collect::<Vec<_>>()
            })
            .filter(|ixs| ixs.iter().all(|i| valid_index(i, &grid)))
            .map(|ixs| ixs.iter().map(|i| to_letter(i, &grid)).collect::<Option<Vec<Letter>>>())
            .filter(|word: &Option<Vec<Letter>>| word.as_ref().is_some_and(|w| diagonals_ma(w)))
            .count();

    Some(answer as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
