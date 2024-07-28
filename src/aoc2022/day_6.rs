use std::collections::HashSet;

fn input() -> String {
	std::fs::read_to_string("input/aoc2022/day_6.txt").unwrap()
}

fn index_of_unique_charset(len: usize) -> usize {
	input()
		.chars()
		.collect::<Vec<_>>()
		.windows(len)
		.enumerate()
		.find(|(_, word)| word.iter().collect::<HashSet<_>>().len() == len)
		.unwrap()
		.0 + len
}

pub fn tuning_trouble_part_1() -> usize {
	index_of_unique_charset(4)
}

pub fn tuning_trouble_part_2() -> usize {
	index_of_unique_charset(14)
}
