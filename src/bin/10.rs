advent_of_code::solution!(10);
use ascent::{ascent_par, ascent_run};
use ascent::aggregators::count;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Position {
    row: u32,
    column: u32
}

fn adjacent_position(position: &Position, other: &Position) -> bool {
    let up = position.row > 0 && position.row - 1 == other.row && position.column == other.column;
    let down = position.row + 1 == other.row && position.column == other.column;
    let left = position.column > 0 && position.row == other.row && position.column - 1 == other.column;
    let right = position.row == other.row && position.column + 1 == other.column;

    up || down || left || right
}

ascent_par! {
    relation node(Position, u32);
    relation edge(Position, Position);

    relation trailfinish(Position);
    relation reachable_finish_part_1(Position, Position);
    relation reachable_finish_part_2(Position, Position, Vec<Position>);
    relation path(Position, Position, Vec<Position>);
    relation adjacent(Position, Position);
    relation trailhead(Position);
    relation score_part_1(Position, usize);
    relation score_part_2(Position, usize);

    trailfinish(p) <-- node(p, 9);
    trailhead(p) <-- node(p, 0);
    adjacent(p_0, p_1) <-- node(p_0, _), node(p_1, _), if adjacent_position(p_0, p_1);
    edge(p_0, p_1) <-- node(p_0, h_0), node(p_1, h_1), adjacent(p_0, p_1), if *h_0 + 1 == *h_1;

    path(p_0, p_1, vec![p_0.clone(), p_1.clone()]) <-- edge(p_0, p_1);
    path(p_0, p_2, prepend(p_0, postfix)) <-- edge(p_0, p_1), path(p_1, p_2, postfix);

    reachable_finish_part_1(start, finish) <-- trailhead(start), path(start, finish, _), trailfinish(finish);
    reachable_finish_part_2(start, finish, p) <-- trailhead(start), path(start, finish, p), trailfinish(finish);

    score_part_1(p, s) <--
        trailhead(p),
        agg s = count() in reachable_finish_part_1(p, _);

    score_part_2(p, s) <--
        trailhead(p),
        agg s = count() in reachable_finish_part_2(p, _, _);

}


pub fn part_one(input: &str) -> Option<u32> {
    let nodes = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                if c == '.' {
                    None
                } else {
                    Some((Position { row: row as u32, column: column as u32 }, c.to_digit(10).unwrap()))
                }
            })
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<(Position, u32)>>();

    let mut prog = AscentProgram::default();
    prog.node.extend(nodes);
    prog.run();

    Some(prog.score_part_1.iter().map(|x| x.1 as u32).sum())
}

fn prepend(pos: &Position, vector: &Vec<Position>)-> Vec<Position> {
    let mut new_vector = vec![pos.clone()];
    new_vector.extend(vector.iter().cloned());
    new_vector
}

pub fn part_two(input: &str) -> Option<u32> {
    let nodes = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                if c == '.' {
                    None
                } else {
                    Some((Position { row: row as u32, column: column as u32 }, c.to_digit(10).unwrap()))
                }
            })
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<(Position, u32)>>();

    let mut prog = AscentProgram::default();
    prog.node.extend(nodes);
    prog.run();

    Some(prog.score_part_2.iter().map(|x| x.1 as u32).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
