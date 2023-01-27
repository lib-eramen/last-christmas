fn calories_per_elf() -> Vec<u64> {
	std::fs::read_to_string("res/aoc2022/day_1.txt")
		.unwrap()
		.split("\n\n")
		.map(|elf| elf
			.lines()
			.map(|cal| cal.parse::<u64>().unwrap())
			.collect::<Vec<u64>>()
			.into_iter()
			.sum::<u64>())
		.collect()
}

pub fn calorie_counting_part_1() -> u64 {
	*calories_per_elf()
		.iter()
		.max()
		.unwrap()
}

pub fn calorie_counting_part_2() -> u64 {
	let mut calories = calories_per_elf();

	calories.sort();
	calories.reverse();

	calories[0..3].iter().sum()
}