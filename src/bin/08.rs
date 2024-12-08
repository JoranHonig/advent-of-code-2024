use itertools::{iproduct, Itertools};

advent_of_code::solution!(8);

fn translation(index: (usize, usize), other: (usize, usize)) -> (i32, i32) {
    (index.0 as i32 - other.0 as i32, index.1 as i32 - other.1 as i32)
}

fn valid(index: (i32, i32), dimension: (usize, usize)) -> bool {
    index.0 >= 0 && index.0 < dimension.0 as i32 && index.1 >= 0 && index.1 < dimension.1 as i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let frequencies = input.chars().filter(|c| *c != '.' && *c != '\n').collect::<std::collections::HashSet<_>>();
    let antenna_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let x = antenna_map.len() ;
    let y = antenna_map[0].len() ;

    let mut anti_nodes: Vec<(u32, u32)> = frequencies
        .iter()
        .map(|frequency| iproduct!(0..x, 0..y).filter(|(i, j)| antenna_map[*i][*j] == *frequency).collect_vec())
        .flat_map(|frequency_locations| frequency_locations.into_iter().combinations(2))
        .flat_map(|combination| {
            let translation_1 = translation(combination[0], combination[1]);
            let translation_2 = translation(combination[1], combination[0]);
            let anti_node_1 = (combination[0].0 as i32 + translation_1.0, combination[0].1 as i32 + translation_1.1);
            let anti_node_2 = (combination[1].0 as i32 + translation_2.0, combination[1].1 as i32 + translation_2.1);

            let mut anti_nodes = Vec::new();
            if valid(anti_node_1, (x, y)) {
                anti_nodes.push((anti_node_1.0 as u32, anti_node_1.1 as u32));
            }

            if valid(anti_node_2, (x, y)) {
                anti_nodes.push((anti_node_2.0 as u32, anti_node_2.1 as u32));
            }

            anti_nodes
        })
        .collect_vec();

    anti_nodes.sort();
    anti_nodes.dedup();

    Some(anti_nodes.len() as u32)
}


pub fn part_two(input: &str) -> Option<u32> {
    let frequencies = input.chars().filter(|c| *c != '.' && *c != '\n').collect::<std::collections::HashSet<_>>();
    let antenna_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let x = antenna_map.len() ;
    let y = antenna_map[0].len() ;

    let mut anti_nodes: Vec<(u32, u32)> = frequencies
        .iter()
        .map(|frequency| iproduct!(0..x, 0..y).filter(|(i, j)| antenna_map[*i][*j] == *frequency).collect_vec())
        .flat_map(|frequency_locations| frequency_locations.into_iter().combinations(2))
        .flat_map(|combination| {
            let translation_1 = translation(combination[0], combination[1]);
            let translation_2 = translation(combination[1], combination[0]);
            let mut anti_node_1 = (combination[0].0 as i32, combination[0].1 as i32);
            let mut anti_node_2 = (combination[1].0 as i32, combination[1].1 as i32);

            let mut anti_nodes = Vec::new();
            while valid(anti_node_1, (x, y)) {
                anti_nodes.push((anti_node_1.0 as u32, anti_node_1.1 as u32));
                anti_node_1 = (anti_node_1.0 + translation_1.0, anti_node_1.1 + translation_1.1);
            }

            while valid(anti_node_2, (x, y)) {
                anti_nodes.push((anti_node_2.0 as u32, anti_node_2.1 as u32));
                anti_node_2 = (anti_node_2.0 + translation_2.0, anti_node_2.1 + translation_2.1);
            }

            anti_nodes
        })
        .collect_vec();

    anti_nodes.sort();
    anti_nodes.dedup();

    Some(anti_nodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
