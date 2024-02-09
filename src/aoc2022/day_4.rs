struct Assignment {
	pub start: u64,
	pub end: u64,
}

fn assignment_pair_overlaps(pair: &Vec<Assignment>) -> bool {
	fn compare(x: &Assignment, y: &Assignment) -> bool {
		x.start <= y.start && x.end >= y.end
	}
	compare(&pair[0], &pair[1]) || compare(&pair[1], &pair[0])
}

fn to_assignment_pair(s: String) -> Vec<Assignment> {
	s.split(',')
		.map(|assignment| {
			let indices = assignment
				.split('-')
				.map(str::parse::<u64>)
				.map(Result::unwrap)
				.collect::<Vec<_>>();
			Assignment {
				start: indices[0],
				end: indices[1],
			}
		})
		.collect()
}

fn assignments() -> Vec<Vec<Assignment>> {
	std::fs::read_to_string("res/aoc2022/day_4.txt")
		.unwrap()
		.lines()
		.map(str::to_string)
		.map(to_assignment_pair)
		.collect()
}

pub fn camp_cleanup_part_1() -> usize {
    assignments()
        .into_iter()
        .filter(assignment_pair_overlaps)
        .count()
}
