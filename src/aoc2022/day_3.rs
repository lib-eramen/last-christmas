use std::collections::HashSet;

type Rucksack = Vec<u8>;
type RucksackHalves = (Vec<u8>, Vec<u8>);

fn priority_of(item: char) -> u8 {
	"abcdefghijklmnopqrstuvwxyz".find(item.to_lowercase().next().unwrap()).unwrap() as u8
		+ if item.is_lowercase() { 1 } else { 27 }
}

fn halves(rucksack: &Rucksack) -> Option<RucksackHalves> {
	if rucksack.len() % 2 != 0 {
		None
	} else {
		Some((
			rucksack[0..(rucksack.len() / 2)].to_vec(),
			rucksack[(rucksack.len() / 2)..rucksack.len()].to_vec(),
		))
	}
}

fn groups(rucksacks: Vec<Rucksack>) -> Option<Vec<Vec<Rucksack>>> {
	if rucksacks.len() < 3 || rucksacks.len() % 3 != 0 {
		None
	} else {
		Some(
			(0..rucksacks.len())
				.step_by(3)
				.map(|group_idx| {
					(group_idx..(group_idx + 3))
						.map(|idx| rucksacks[idx].clone())
						.collect::<Vec<Rucksack>>()
				})
				.collect(),
		)
	}
}

fn to_rucksack(s: String) -> Rucksack {
	s.chars().map(priority_of).collect()
}

fn intersection_between_compartments(rucksack: &Rucksack) -> Option<u8> {
	let halves = halves(rucksack).unwrap();
	let mut item_set: HashSet<(bool, u8)> =
		HashSet::from_iter(halves.0.into_iter().map(|item| (true, item)));

	for item in halves.1 {
		if item_set.contains(&(true, item)) {
			return Some(item);
		}
		item_set.insert((false, item));
	}

	None
}

fn rucksack_intersection(a: &Rucksack, b: &Rucksack) -> Vec<u8> {
	let set_a: HashSet<&u8> = HashSet::from_iter(a.iter());
	let set_b: HashSet<&u8> = HashSet::from_iter(b.iter());
	set_a.intersection(&set_b).map(|item| **item).collect()
}

fn three_way_intersection(rucksacks: Vec<&Rucksack>) -> Option<u8> {
	let (a, b, c) = (rucksacks[0], rucksacks[1], rucksacks[2]);
	let (ab, bc) = (rucksack_intersection(a, b), rucksack_intersection(b, c));
	let set_ab: HashSet<&u8> = HashSet::from_iter(ab.iter());
	let set_bc: HashSet<&u8> = HashSet::from_iter(bc.iter());
	set_ab.intersection(&set_bc).next().map(|item| **item)
}

fn rucksacks() -> Vec<Rucksack> {
	std::fs::read_to_string("res/aoc2022/day_3.txt")
		.unwrap()
		.lines()
		.map(str::to_string)
		.map(to_rucksack)
		.collect()
}

pub fn rucksack_reorganization_part_1() -> u64 {
	rucksacks()
		.iter()
		.map(intersection_between_compartments)
		.map(Option::unwrap)
		.map(|item| item as u64)
		.sum()
}

pub fn rucksack_reorganization_part_2() -> u64 {
	groups(rucksacks())
		.unwrap()
		.iter()
		.map(|group| group.iter().collect::<Vec<&Rucksack>>())
		.map(three_way_intersection)
		.map(Option::unwrap)
		.map(|item| item as u64)
		.sum()
}
