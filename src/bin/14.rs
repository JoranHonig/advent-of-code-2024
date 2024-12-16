use std::{collections::HashMap, path::Display};
use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(14);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Robot {
    position: (u32, u32),
    velocity: (i32, i32),
}


impl TryFrom<&str> for Robot {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let captures = pattern.captures(value).ok_or(())?;
        let position = (captures[1].parse::<u32>().map_err(|_| ())?, captures[2].parse::<u32>().map_err(|_| ())?);
        let velocity = (captures[3].parse::<i32>().map_err(|_| ())?, captures[4].parse::<i32>().map_err(|_| ())?);
        Ok(Robot { position, velocity })

    }
}


impl Robot {
    fn step(&self, bounds: (u32, u32)) -> Self {
        let new_position = ((self.position.0 as i32 + self.velocity.0).rem_euclid(bounds.0 as i32) as u32, (self.position.1 as i32 + self.velocity.1).rem_euclid(bounds.1 as i32) as u32);
        Robot { position: new_position, velocity: self.velocity }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Map {
    robots: Vec<Robot>,
}


impl Map {
    fn memoized_step(&self, bounds: (u32, u32), memory: &mut HashMap<Map, Map>) -> Map {
        if let Some(result) = memory.get(self) {
            println!("cache hit");
            result.clone()
        } else {
            let result = Map { robots: self.robots.iter().map(|robot| robot.step(bounds)).collect() };
            memory.insert(self.clone(), result.clone());
            result
        }
    }

    fn print_map(&self, bounds: (u32, u32)) {
        let mut visual = vec![vec!['.'; bounds.0 as usize]; bounds.1 as usize];

        for robot in &self.robots {
            visual[robot.position.1 as usize][robot.position.0 as usize] = '#';
        }

        for row in visual {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn no_overlap(&self) -> bool {
        self.robots.iter().map(|robot| robot.position).collect::<Vec<(u32, u32)>>().into_iter().unique().count() == self.robots.len()
    }


    fn robots_divided_by_quadrant(&self, bounds: (u32, u32)) -> Vec<Vec<Robot>> {

        self.robots.iter().fold(
            vec![vec![]; 4],
            |mut acc, robot| {
                let quadrant = if robot.position.0 < bounds.0 / 2 && robot.position.1 < bounds.1 / 2 {
                    Some(0)
                } else if robot.position.0 > bounds.0 / 2 && robot.position.1 < bounds.1 / 2 {
                    Some(1)
                } else if robot.position.0 < bounds.0 / 2 && robot.position.1 > bounds.1 / 2 {
                    Some(2)
                } else if robot.position.0 > bounds.0 / 2 && robot.position.1 > bounds.1 / 2 {
                    Some(3)
                } else {
                    None
                };
                if let Some(quadrant) = quadrant {
                    acc[quadrant].push(robot.clone());
                }
                acc
            }
        )
    }
}

impl TryFrom<&str> for Map {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let robots = value.lines().map(|line| Robot::try_from(line)).collect::<Result<Vec<Robot>, ()>>()?;
        Ok(Map { robots })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::try_from(input).ok()?;

    // set bounds dependent on example imput vs real input
    let bounds;
    if map.robots.len() == 12 {
        bounds = (11, 7);
    } else {
        bounds = (101, 103);
    }

    let stepped = (0..100).fold(map, |map, _| map.memoized_step(bounds, &mut HashMap::new()));

    let result =
        stepped
            .robots_divided_by_quadrant(bounds)
            .iter()
            .map(|quadrant| quadrant.len() as u32)
            .product();


    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::try_from(input).ok()?;

    // set bounds dependent on example imput vs real input
    let bounds;
    if map.robots.len() == 12 {
        bounds = (11, 7);
    } else {
        bounds = (101, 103);
    }

    let mut memory = HashMap::new();

    for i in 1..10000 {
        map = map.memoized_step(bounds, &mut memory);
        if map.no_overlap() {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
