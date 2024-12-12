advent_of_code::solution!(12);

use std::collections::BTreeSet;

use ascent::{ascent_par, ascent_run, ascent_run_par};
use ascent::lattice::set::Set;
use ascent::aggregators::{count, sum};
use ascent_byods_rels::trrel_uf;

use itertools::Itertools;


#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Position {
    row: i32,
    column: i32
}

fn adjacent_position(position: &Position, other: &Position) -> bool {
    let up = position.row > 0 &&      position.row - 1 == other.row  && position.column == other.column;
    let down =                        position.row + 1 == other.row  && position.column == other.column;
    let left = position.column > 0 && position.row == other.row      && position.column - 1 == other.column;
    let right =                       position.row == other.row      && position.column + 1 == other.column;

    up || down || left || right
}

fn crosswise_adjacent_position(position: &Position, other: &Position) -> bool {
        let up_left = position.row > 0 && position.column > 0 && position.row - 1 == other.row && position.column - 1 == other.column;
        let up_right = position.row > 0 && position.row - 1 == other.row && position.column + 1 == other.column;
        let down_left = position.column > 0 && position.row + 1 == other.row && position.column - 1 == other.column;
        let down_right = position.row + 1 == other.row && position.column + 1 == other.column;

        up_left || up_right || down_left || down_right
    }

ascent_par!{
    relation plot(Position, char);
    lattice region(Position, char, Set<Position>);
    relation unique_region(char, Set<Position>);
    relation region_info(Set<Position>, usize, usize);

    relation area(Set<Position>, usize);
    relation circumference(Set<Position>, usize);
    relation faces(Set<Position>, usize);

    relation adjacent(Position, Position);
    relation cross_adjacent(Position, Position);

    // relation external(Set<Position>, Position);
    // relation internal(Set<Position>, Position);
    relation adjacent_pair_in_region(Position, Position, Set<Position>);
    relation shared_external_pair(Position, Position, Position, Set<Position>);
    relation counted_shared_external_pair(Set<Position>, Position, usize);

    adjacent(one, other) <-- plot(one, _), plot(other, _) if adjacent_position(one, other);
    cross_adjacent(one, other) <-- plot(one, _), plot(other, _) if crosswise_adjacent_position(one, other);

    region(p, c, Set::singleton(*p)) <-- plot(p, c);
    region(p, c, other_set) <-- plot(p, c), adjacent(p, neighbour), plot(neighbour, c), region(neighbour, c, other_set);

    unique_region(c, set) <-- region(_, c, set);

}

pub fn part_one(input: &str) -> Option<u32> {
    let mut prog = AscentProgram::default();
    prog.plot = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                (Position { row: row as i32, column: column as i32}, c)
            })
        })
        .collect();

    prog.run();
    println!("{}", prog.scc_times_summary());
    println!("step 1 finished");

    let result = ascent_run_par! {
        relation plot(Position, char) = prog.plot.clone();
        relation unique_region(char, Set<Position>) = prog.unique_region.clone();

        relation region_info(Set<Position>, usize, usize);
        relation area(Set<Position>, usize);
        relation circumference(Set<Position>, usize);

        relation adjacent_pair_in_region(Position, Position, Set<Position>);
        relation adjacent(Position, Position) = prog.adjacent.clone();

        area(set, set.len()) <--
            unique_region(_, set);

        adjacent_pair_in_region(one, other, set) <--
            unique_region(_, set), adjacent(one, other) if set.contains(one) && set.contains(other);

        circumference(set, area * 4 - adjacent_pairs) <--
            unique_region(_, set),
            area(set, area),
            agg adjacent_pairs = count() in adjacent_pair_in_region(_, _, set);

        region_info(set, area, circumference) <--
            unique_region(_, set),
            area(set, area),
            circumference(set, circumference);

    };

    let result = result.region_info.iter().map(|(_, area, circumference)| area * *circumference as usize).sum::<usize>() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut prog = AscentProgram::default();
    prog.plot = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                (Position { row: row as i32, column: column as i32 }, c)
            })
        })
        .collect();

    prog.run();

    let max_row = input.lines().count() as i32 - 1;
    let max_column = input.lines().next().unwrap().chars().count() as i32 - 1;

    let result = ascent_run_par!{
        #![measure_rule_times]
        relation plot(Position, char) = prog.plot.clone();
        relation unique_region(char, Set<Position>) = prog.unique_region.clone();

        relation region_info(Set<Position>, usize, usize);
        relation area(Set<Position>, usize);

        relation adjacent_pair_in_region(Position, Position, Set<Position>);
        relation adjacent(Position, Position);

        relation faces(Set<Position>, usize);
        relation edge(Position, Position);


        relation external(Set<Position>, Position);
        relation internal(Set<Position>, Position);
        relation shared_external_pair(Position, Position, Position, Set<Position>);
        relation counted_shared_external_pair(Set<Position>, Position, usize);

        adjacent_pair_in_region(one, other, set) <--
            unique_region(_, set), adjacent(one, other) if set.contains(one) && set.contains(other);

        adjacent(one, other) <-- plot(one, _), plot(other, _), if adjacent_position(one, other);
        adjacent(one, other) <-- edge(one, _), edge(other, _), if adjacent_position(one, other);
        adjacent(one, other) <-- edge(_, one), edge(_, other), if adjacent_position(one, other);

        area(set, set.len()) <--
            unique_region(_, set);

        edge(one, *other) <-- plot(one, c), plot(other, c_), adjacent(one, other), if c != c_;
        edge(one, Position{row: one.row - 1, column: one.column}) <-- plot(one, _), if one.row == 0;
        edge(one, Position{row: one.row, column: one.column - 1}) <-- plot(one, _), if one.column == 0;
        edge(one, Position{row: one.row + 1, column: one.column}) <-- plot(one, _), if one.row == max_row;
        edge(one, Position{row: one.row, column: one.column + 1}) <-- plot(one, _), if one.column == max_column;


        lattice region_edges(Position, Set<(Position, Position)>);
        region_edges(p, Set::singleton((*p, *o))) <-- edge(p, o);
        region_edges(p, set) <--
            plot(p, c),
            plot(other, c),
            if other != p ,
            unique_region(c, region),
            if region.contains(p) && region.contains(other),
            region_edges(other, set);

        relation actual_region_edges(Set<Position>, Set<(Position, Position)>);
        actual_region_edges(region, edges) <--
            unique_region(_, region),
            plot(node, _),
            if region.contains(node),
            region_edges(node, edges);

        relation region_edge(Set<Position>, Position, Position);
        region_edge(region, p, q) <--
            actual_region_edges(region, edges),
            edge(p, q),
            if edges.contains(&(*p, *q));

        lattice face(Position, Position, Set<Position>, Set<(Position, Position)>);
        face (p, q, region, Set::singleton((*p, *q))) <--
            region_edge(region, p, q);

        face (p, q, region, face_edges) <--
            region_edge(region, p, q),
            region_edge(region, x, y),
            adjacent(p, x),
            adjacent(q, y),
            face(x, y, region, face_edges);

        relation region_face(Set<Position>, Set<(Position, Position)>);
        region_face(region, face) <--
            face(_, _, region, face);

        relation region_faces(Set<Position>, usize); //Set<(Position, Position)>);

        region_faces(region, c) <--
            actual_region_edges(region, region_edges),
             agg c = count() in region_face(region, _);
    };

    let result = result.region_faces.iter().map(|(region, faces_count)| region.iter().count() as u32 * *faces_count as u32).sum::<u32>();

   Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
