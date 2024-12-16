use rayon::iter::split;

advent_of_code::solution!(15);


use advent_of_code::define_nodes;
use advent_of_code::utils::map::{Map, BoxedMap, Direction, Position};

define_nodes!(
    '#' => Obstacle,
    '.' => Empty,
    '@' => Robot,
    'O' => Box,
    '[' => BoxL,
    ']' => BoxR,
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RobotMap {
    pub base: BoxedMap<Node>,
    robot_location: Position,
}
fn gps_coordinate(position: &Position) -> u32 {
   position.y * 100 + position.x
}

impl RobotMap {
    fn score(&self) -> u32 {
        self.base
            .position_iter()
            .filter(|p| [Node::Box, Node::BoxL].contains(self.base.get(&p).unwrap()))
            .map(|b| gps_coordinate(&b))
            .sum()
    }

    fn first_empty_in_direction(&self, position: &Position, direction: &Direction) -> Option<Position> {
        let mut position = *position;
        loop {
            position = self.base.neighbour_in_direction(position, *direction)?;
            if self.base.get(&position)? == &Node::Empty {
                return Some(position);
            } else if self.base.get(&position)? == &Node::Obstacle {
                return None;
            }
        }
    }

    fn try_push(&mut self, position: &Position, direction: &Direction, other: bool) -> Result<(), ()> {
        let next_position = self.base.neighbour_in_direction(*position, *direction).ok_or(())?;
        let current_type = self.base.get(&position).unwrap().clone();

        let is_self_box = [Node::Box, Node::BoxL, Node::BoxR].contains(&current_type);
        if  is_self_box && [Direction::Up, Direction::Down].contains(direction) & !other {
            let other = match current_type {
                Node::BoxL => Some(self.base.neighbour_in_direction(*position, Direction::Right).unwrap()),
                Node::BoxR => Some(self.base.neighbour_in_direction(*position, Direction::Left).unwrap()),
                _ => None,
            };

            if let Some(other) = other {
                self.try_push(&other, direction, true)?;
            }
        }
        let next_type = self.base.get(&next_position).unwrap().clone();

        // handle edge cases
        let is_obstacle = next_type == Node::Obstacle;
        let is_empty = next_type == Node::Empty;

        // case 1: obstacle
        if is_obstacle {
            return Err(());
        }

        // case 2: empty
        if is_empty {
            self.base.swap(&position, &next_position);
            return Ok(());
        }

        // case 3: box
        let is_box = [Node::Box, Node::BoxL, Node::BoxR].contains(&self.base.get(&next_position).unwrap());
        if is_box {
            self.try_push(&next_position, direction, false)?;
            self.base.swap(&position, &next_position);
        }


        Ok(())
    }


    fn move_robot(&mut self, direction: &Direction, part_1: bool) {
        let position = self.base.neighbour_in_direction(self.robot_location, *direction);

        if position.is_none() {
            return;
        }

        let position = position.unwrap();

        // simple move without moving obstacles
        let is_empty = self.base.get(&position).unwrap() == &Node::Empty;

        if is_empty {
            self.base.swap(&self.robot_location, &position);
            self.robot_location = position;
        }

        // complex move if there is an obstacle
        let is_box = [Node::Box, Node::BoxL, Node::BoxR].contains(&self.base.get(&position).unwrap());

        if is_box  && part_1 {
            let next_position = self.first_empty_in_direction(&position, direction);
            if next_position.is_none() {
                return;
            }

            let next_position = next_position.unwrap();
            self.base.swap(&position, &next_position);
            self.base.swap(&self.robot_location, &position);
            self.robot_location = position;
        } else if is_box {
            let mut new_self = self.clone();

            let success = new_self.try_push(&position, direction, false).is_ok();
            if success {
                self.base = new_self.base;
                self.base.swap(&self.robot_location, &position);
                self.robot_location = position;
            }

        }
    }

}

impl TryFrom<&str> for RobotMap {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = value
            .lines()
            .map(|line| line.chars().map(|c| Node::try_from(c)).collect::<Result<Vec<Node>, ()>>())
            .collect::<Result<Vec<Vec<Node>>, ()>>()?;


        let base = BoxedMap::try_from(data)?;

        let robot_location = base.position_iter().find(|p| {base.get(p).unwrap() == &Node::Robot}).ok_or(())?;

        Ok(Self { base, robot_location })
    }
}

fn try_parse_move(value: &char) -> Result<Direction, ()> {
    match value {
        '^' => Ok(Direction::Up),
        'v' => Ok(Direction::Down),
        '<' => Ok(Direction::Left),
        '>' => Ok(Direction::Right),
        _ => {
            println!("Invalid move: {}", value);
            Err(())
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let split_input = input.split("\n\n").collect::<Vec<&str>>();

    let mut grid = RobotMap::try_from(*split_input.get(0)?).ok()?;
    let moves = split_input.get(1)?.trim().chars().filter(|c| *c != '\n').map(|c| try_parse_move(&c)).collect::<Result<Vec<Direction>, ()>>().ok()?;


    moves.iter().for_each(|direction| grid.move_robot(direction, true));
    // println!("{:?}", grid.base);

    Some(grid.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let split_input = input.split("\n\n").collect::<Vec<&str>>();

    let new_map = split_input
        .get(0)?
        .lines()
        .map(|line| {
            line.chars().map(|c| {
                match c {
                    '#' => "##",
                    '.' => "..",
                    '@' => "@.",
                    'O' => "[]",
                    _ => panic!("Invalid character: {}", c),
                }
            }).collect::<String>() + "\n"
        }).collect::<String>();

    let mut grid = RobotMap::try_from(new_map.as_str()).ok()?;
    let moves = split_input.get(1)?.trim().chars().filter(|c| *c != '\n').map(|c| try_parse_move(&c)).collect::<Result<Vec<Direction>, ()>>().ok()?;

    moves.iter().for_each(|direction| grid.move_robot(direction, false));

    Some(grid.score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
