#[derive(PartialEq, Eq, Debug, Clone)]
enum Play {
	Rock,
	Paper,
	Scissor,
}

enum Outcome {
	Win,
	Draw,
	Lose,
}

impl Play {
	fn _value(&self) -> u8 {
		match self {
			Play::Rock => 0,
			Play::Paper => 1,
			Play::Scissor => 2,
		}
	}

	pub fn interpret_part_1(c: char) -> Self {
		match c {
			'A' | 'X' => Self::Rock,
			'B' | 'Y' => Self::Paper,
			'C' | 'Z' => Self::Scissor,
			_ => panic!("Illegal string argument"),
		}
	}

	// Might have been unnecessary, but oh well, foolproof
	pub fn interpret_part_2(c: char) -> Self {
		match c {
			'A' => Self::Rock,
			'B' => Self::Paper,
			'C' => Self::Scissor,
			_ => panic!("Illegal string argument"),
		}
	}

	pub fn interacted_to(&self, out: Outcome) -> Self {
		match out {
			Outcome::Draw => self.clone(),
			Outcome::Lose => match self {
				Play::Rock => Play::Scissor,
				Play::Paper => Play::Rock,
				Play::Scissor => Play::Paper,
			},
			Outcome::Win => match self {
				Play::Rock => Play::Paper,
				Play::Paper => Play::Scissor,
				Play::Scissor => Play::Rock,
			},
		}
	}

	pub fn cmp(you: &Self, them: &Self) -> Outcome {
		if you == them {
			Outcome::Draw
		} else {
			match you {
				Play::Rock => {
					if them == &Play::Scissor {
						Outcome::Win
					} else {
						Outcome::Lose
					}
				},
				Play::Paper => {
					if them == &Play::Rock {
						Outcome::Win
					} else {
						Outcome::Lose
					}
				},
				Play::Scissor => {
					if them == &Play::Paper {
						Outcome::Win
					} else {
						Outcome::Lose
					}
				},
			}
		}
	}

	pub fn value(&self) -> u8 {
		match self {
			Play::Rock => 1,
			Play::Paper => 2,
			Play::Scissor => 3,
		}
	}
}

impl Outcome {
	pub fn score(&self) -> u8 {
		match self {
			Outcome::Win => 6,
			Outcome::Draw => 3,
			Outcome::Lose => 0,
		}
	}

	pub fn interpret_part_2(c: char) -> Self {
		match c {
			'X' => Outcome::Lose,
			'Y' => Outcome::Draw,
			'Z' => Outcome::Win,
			_ => panic!("Illegal string argument"),
		}
	}
}

#[derive(Debug)]
struct Match {
	pub you: Play,
	pub them: Play,
}

impl Match {
	pub fn interpret_part_1(literal: &str) -> Self {
		Self {
			you: Play::interpret_part_1(literal.chars().nth(2).unwrap()),
			them: Play::interpret_part_1(literal.chars().next().unwrap()),
		}
	}

	pub fn interpret_part_2(literal: &str) -> Self {
		let them = Play::interpret_part_2(literal.chars().next().unwrap());
		Self {
			you: them.interacted_to(Outcome::interpret_part_2(literal.chars().nth(2).unwrap())),
			them,
		}
	}

	pub fn outcome(&self) -> Outcome {
		Play::cmp(&self.you, &self.them)
	}

	pub fn score(&self) -> u8 {
		self.you.value() + self.outcome().score()
	}
}

fn match_literals() -> Vec<String> {
	std::fs::read_to_string("res/aoc2022/day_2.txt").unwrap().lines().map(str::to_string).collect()
}

pub fn rock_paper_scissors_part_1() -> u64 {
	match_literals()
		.iter()
		.map(String::as_str)
		.map(Match::interpret_part_1)
		.map(|m| m.score() as u64)
		.sum()
}

pub fn rock_paper_scissors_part_2() -> u64 {
	match_literals()
		.iter()
		.map(String::as_str)
		.map(Match::interpret_part_2)
		.map(|m| m.score() as u64)
		.sum()
}
