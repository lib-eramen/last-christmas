use std::collections::HashSet;

pub fn tuning_trouble_part_1() -> usize {
	std::fs::read_to_string("input/aoc2022/day_6.txt")
		.unwrap()
		.chars()
		.collect::<Vec<_>>()
		.windows(4)
		.enumerate()
		.find(|(_, word)| word.iter().collect::<HashSet<_>>().len() == 4)
		.unwrap()
		.0 + 4
}
