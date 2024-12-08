advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Map {
    dimensions: (u32, u32),
    position: (u32, u32),
    orientation: Direction,
    content: Vec<Vec<char>>,
    visited: Vec<(u32, u32)>,
    path: Vec<(u32, u32, Direction)>,
}

impl Map {
    fn turn_right(&mut self) {
        self.orientation = match self.orientation {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn in_loop(&self) -> bool {
        self.path.contains(&(self.position.0, self.position.1, self.orientation))
    }

    fn step(&mut self) -> Option<(u32, u32)> {
        if self.in_loop() {
            self.path.pop(); // Remove the last step
            return None;
        }

        let (x, y) = (self.position.0 as i32, self.position.1 as i32);
        self.visited.push((self.position.0, self.position.1));
        self.path.push((self.position.0, self.position.1, self.orientation));


        let next_position = match self.orientation {
            Direction::North => (x - 1, y),
            Direction::East => (x, y + 1),
            Direction::South => (x + 1, y),
            Direction::West => (x, y - 1),
        };

        let valid = next_position.0 >= 0 && next_position.0 < self.dimensions.0 as i32 &&
                    next_position.1 >= 0 && next_position.1 < self.dimensions.1 as i32;


        if !valid {
            self.path.pop();
            return None;
        }

        if self.content[next_position.0 as usize][next_position.1 as usize] == '#' {
            self.turn_right();
            return Some(self.position);
        }

        self.position = (next_position.0 as u32, next_position.1 as u32);
        Some((next_position.0 as u32, next_position.1 as u32))
    }

}

impl TryFrom<&str> for Map {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let x = input.lines().count();
        let y = input.lines().next().ok_or(())?.len();

        let dimensions = (x as u32, y as u32);

        let mut location: Option<(u32, u32)> = None;
        let content = input.lines().map(|line| line.chars().collect()).collect();

        for (col, line) in input.lines().enumerate() {
            for (row, value) in line.chars().enumerate() {
                if value == '^' {
                    location = Some((col as u32, row as u32));
                    break;
                }
            }
        }

        Ok(Map {
            dimensions,
            position: location.ok_or(())?,
            orientation: Direction::North,
            content,
            visited: Vec::new(),
            path: Vec::new(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::try_from(input).ok()?;
    while let Some(_) = map.step() {}
    let mut visited = map.visited.clone();
    visited.sort();
    visited.dedup();

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::try_from(input).ok()?;

    while let Some(_) = map.step() {}
    let mut visited = map.visited.clone();
    visited.sort();
    visited.dedup();

    let fresh_map = Map::try_from(input).ok()?;
    let mut loop_locations: Vec<(u32, u32)> = vec![];

    for location in visited.iter() {
        let mut map = fresh_map.clone();
        map.content[location.0 as usize][location.1 as usize] = '#';

        while let Some(_) = map.step() {}

        if map.in_loop() {
            loop_locations.push(*location);
        }
    }

    Some(loop_locations.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
