use std::collections::HashSet;

type Rucksack = (Vec<u8>, Vec<u8>);

fn halves(s: String) -> Option<(String, String)> {
	if s.len() % 2 != 0 {
		None
	} else {
		Some((
			s[0..(s.len() / 2)].to_string(),
			s[(s.len() / 2)..s.len()].to_string(),
		))
	}
}

fn priority_of(item: char) -> u8 {
	"abcdefghijklmnopqrstuvwxyz".find(item.to_lowercase().next().unwrap()).unwrap() as u8
		+ if item.is_lowercase() { 1 } else { 27 }
}

fn item_in_both_compartments(rucksack: &Rucksack) -> Option<u8> {
	let rucksack = rucksack.clone();
	let mut item_set: HashSet<(bool, u8)> = HashSet::new();
	rucksack.0.into_iter().for_each(|item| { item_set.insert((true, item)); });

	for item in rucksack.1 {
		if item_set.contains(&(true, item)) {
			return Some(item);
		}
		item_set.insert((false, item));
	}

	None
}

fn rucksacks() -> Vec<Rucksack> {
	std::fs::read_to_string("res/aoc2022/day_3.txt")
		.unwrap()
		.lines()
		.map(str::to_string)
		.map(halves)
		.map(Option::unwrap)
		.map(|(c1, c2)| (
			c1.chars().map(priority_of).collect(),
			c2.chars().map(priority_of).collect(),
		))
		.collect()
}

pub fn rucksack_reorganization_part_1() -> u64 {
	rucksacks().iter().map(item_in_both_compartments).map(|i| i.unwrap() as u64).sum()
}
