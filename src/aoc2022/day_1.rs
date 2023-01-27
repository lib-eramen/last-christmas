pub fn calorie_counting() {
	println!(
		"{}",
		std::fs::read_to_string("res/aoc2022/day_1.txt")
			.unwrap()
			.split("\n\n")
			.map(|elf| elf
				.lines()
				.map(|cal| cal.parse::<u64>().unwrap())
				.collect::<Vec<u64>>()
				.into_iter()
				.sum::<u64>())
			.max()
			.unwrap()
	);
}
