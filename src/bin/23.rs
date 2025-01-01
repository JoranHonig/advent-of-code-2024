advent_of_code::solution!(23);

use std::collections::BTreeSet;

use ascent::{ascent_par, ascent_run_par};
use itertools::{self, Itertools};
use ascent::lattice::set::Set;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Computer {
    name: [char; 2]
}

fn parse_input_line(input: &str) -> Result<(Computer, Computer), ()> {
    let (name_0, name_1) = input.split("-").collect_vec().into_iter().collect_tuple().ok_or(())?;
    let computer_0 = Computer { name: name_0.chars().collect_vec().try_into().map_err(|_| ())? };
    let computer_1 = Computer { name: name_1.chars().collect_vec().try_into().map_err(|_| ())? };

    Ok((computer_0, computer_1))
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = ascent_run_par! {
        relation connection(Computer, Computer) = input.lines().map(parse_input_line).collect::<Result<_, _>>().ok()?;
        relation triple(Computer, Computer, Computer);

        // reflexive
        connection(a, b) <-- connection(b, a);

        // compute triples
        triple(a, b, c) <--
            connection(a, b),
            connection(b, c),
            connection(c, a),
            if a != b && b != c && a != c && a < b && b < c, // ordering ensures unique triples
            if [a.name[0], b.name[0], c.name[0]].contains(&'t'); // filters out triples that don't contain a computer starting with 't'
    };

    Some(result.triple.len() as u32)
}

fn immutable_insert<T: Ord + Clone>(set: &BTreeSet<T>, item: T) -> BTreeSet<T> {
    let new_set = BTreeSet::from([item]);
    set.union(&new_set).cloned().collect()
}

pub fn part_two(input: &str) -> Option<String> {
    let dl_result = ascent_run_par! {
        relation connection(Computer, Computer) = input.lines().map(parse_input_line).collect::<Result<_, _>>().ok()?;
        lattice connections(Computer, Set<Computer>);
        relation clique(BTreeSet<Computer>);

        // reflexive
        connection(a, b) <-- connection(b, a);

        // connections
        connections(c, Set::singleton(*o)) <--
            connection(c, o);

        clique(BTreeSet::from([*computer])) <--
            connection(computer, _);

        clique(immutable_insert(set, *computer)) <--
            connections(computer, cons),
            clique(set),
            if !set.contains(computer) && set.is_subset(&cons.0);
    };

    let largest_clique = dl_result.clique.iter().max_by_key(|(set,)| set.len())?;
    let mut computer_names = largest_clique.0.iter().map(|computer| computer.name.iter().collect::<String>()).collect::<Vec<String>>();
    computer_names.sort();
    let password = computer_names.join(",");

    Some(password)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
