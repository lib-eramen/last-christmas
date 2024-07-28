#[derive(Clone, Copy)]
struct Assignment {
	pub start: u64,
	pub end: u64,
}

type AssignmentPair = (Assignment, Assignment);

fn to_assignment_pair(s: String) -> AssignmentPair {
	let vec = s
		.split(',')
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
		.collect::<Vec<_>>();
	(vec[0], vec[1])
}

fn assignments() -> Vec<AssignmentPair> {
	std::fs::read_to_string("input/aoc2022/day_4.txt")
		.unwrap()
		.lines()
		.map(str::to_string)
		.map(to_assignment_pair)
		.collect()
}

fn contains(x: &Assignment, y: &Assignment) -> bool {
	x.start <= y.start && x.end >= y.end
}

fn assignment_pair_fully_contains(pair: &AssignmentPair) -> bool {
	contains(&pair.0, &pair.1) || contains(&pair.1, &pair.0)
}

fn overlaps(x: &Assignment, y: &Assignment) -> bool {
	x.start < y.start && x.end >= y.start
}

fn assignment_pair_overlaps(pair: &AssignmentPair) -> bool {
	overlaps(&pair.0, &pair.1) || overlaps(&pair.1, &pair.0) || assignment_pair_fully_contains(pair)
}

pub fn camp_cleanup_part_1() -> usize {
	assignments().into_iter().filter(assignment_pair_fully_contains).count()
}

pub fn camp_cleanup_part_2() -> usize {
	assignments().into_iter().filter(assignment_pair_overlaps).count()
}
