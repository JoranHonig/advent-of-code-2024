advent_of_code::solution!(16);

use advent_of_code::template::runner::run_part;
use advent_of_code::utils::map::{Map, BoxedMap, Direction, Position};
use ascent::{ascent_run_par};
use ascent::lattice::Dual;
use ascent::lattice::set::Set;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Obstacle,
    Empty,
    Start,
    Finish,
}


impl TryFrom<char> for Node {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Node::Obstacle),
            '.' => Ok(Node::Empty),
            'S' => Ok(Node::Start),
            'E' => Ok(Node::Finish),
            _ => Err(()),
        }
    }
}

fn turn_one(direction: &Direction, reverse: bool) -> Direction {
    if !reverse {
        match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    } else {
        match direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

fn run_ascent(input: &str, part_one: bool) -> Option<u32> {
    let nodes = input
        .lines()
        .map(|line| line.chars().map(|c| Node::try_from(c)).collect::<Result<Vec<Node>, ()>>())
        .collect::<Result<Vec<Vec<Node>>, ()>>()
        .ok()?;

    let node_relation = nodes.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().map(move |(x, node)| {
            (Position { x: x as u32, y: y as u32 }, *node)
        })
    }).collect();

    let map = BoxedMap::try_from(nodes).ok()?;

    let edge_relation = map.position_iter().flat_map(|position| {
        map.neighbours(&position).into_iter().map(move |neighbour| {
            (position, neighbour)
        })
    }).collect();

    let start = map.position_iter().find(|position| map.get(position) == Some(&Node::Start))?;
    let finish = map.position_iter().find(|position| map.get(position) == Some(&Node::Finish))?;

    let result = ascent_run_par!{
        // input relations
        relation node(Position, Node) = node_relation;
        relation directions(Direction);
        directions(Direction::Up);
        directions(Direction::Down);
        directions(Direction::Left);
        directions(Direction::Right);

        relation edge(Position, Position) = edge_relation;
        relation start(Position, Direction) = vec![(start, Direction::Right)].into_iter().collect();
        relation finish(Position) = vec![(finish,)].into_iter().collect();

        // computed relations
        lattice shortest_path(Position, Direction, Position, Direction, Dual<u32>);
        lattice shortest_path_to(Position, Direction, Dual<u32>);
        relation reaches_finish(Position, Direction);

        // output relations
        relation result_part_1(Dual<u32>);
        relation result_part_2(Position);

        // start
        shortest_path(position, direction, position, direction, Dual(0)) <--
            node(position, _),
            start(position, direction); // we compute partial relation bc it's faster

        // turning
        shortest_path(position, direction, position, turn_one(direction, true), Dual(previous_score + 1000)) <--
            shortest_path(_, _, position, direction, ?Dual(previous_score));
        shortest_path(position, direction, position, turn_one(direction, false), Dual(previous_score + 1000)) <--
            shortest_path(_, _, position, direction, ?Dual(previous_score));

        // moving
        shortest_path(position, direction, new_position, direction, Dual(previous_score + 1)) <--
            edge(position, new_position),
            node(new_position, node_type),
            if node_type == &Node::Empty || node_type == &Node::Finish,
            shortest_path(_, _, position, direction, ?Dual(previous_score)),
            if map.neighbour_in_direction(*position, *direction) == Some(*new_position);


        // union paths to only store the shortest one
        shortest_path_to(position, direction, Dual(*score)) <--
            shortest_path(_, _, position, direction, ?Dual(score));

        // reaches finish is a relation that only stores those positions which reache the finish the quickest
        reaches_finish(position, d) <--
            directions(d),
            finish(position);

        // moving
        reaches_finish(position, direction) <--
            reaches_finish(other, direction),
            shortest_path_to(position, direction, ?Dual(original_score)),
            if map.neighbour_in_direction(*position, *direction) == Some(*other),
            shortest_path_to(other, direction, ?Dual(efficient_score)),
            if *original_score  +1 == *efficient_score ;

        // turning
        reaches_finish(position, direction) <--
            shortest_path_to(position, direction, ?Dual(original_score)),
            reaches_finish(position, turn_one(direction, false)),
            shortest_path_to(position, turn_one(direction, false), ?Dual(efficient_score)),
            if *original_score + 1000 == *efficient_score;

        reaches_finish(position, direction) <--
            shortest_path_to(position, direction, ?Dual(original_score)),
            reaches_finish(position, turn_one(direction, true)),
            shortest_path_to(position, turn_one(direction, true), ?Dual(efficient_score)),
            if *original_score + 1000 == *efficient_score;

        // output
        result_part_1(score) <--
            finish(finish_position),
            shortest_path(_, _, finish_position, _, score);
        result_part_2(p) <-- reaches_finish(p, _);
    };

    if part_one {
        let score = result.result_part_1.iter().map(|(Dual(score), )| score).min()?;
        Some(*score)
    } else {
        Some(result.result_part_2.len() as u32)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    run_ascent(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    run_ascent(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
