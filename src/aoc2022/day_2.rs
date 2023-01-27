#[derive(PartialEq, Eq, Debug)]
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

	pub fn from(s: char) -> Self {
		match s {
			'A' | 'X' => Self::Rock,
			'B' | 'Y' => Self::Paper,
			'C' | 'Z' => Self::Scissor,
			_ => panic!("Illegal string argument"),
		}
	}

	pub fn cmp(you: &Self, opponent: &Self) -> Outcome {
		if you == opponent {
			Outcome::Draw
		} else {
			match you {
				Play::Rock => {
					if opponent == &Play::Scissor {
						Outcome::Win
					} else {
						Outcome::Lose
					}
				},
				Play::Paper => {
					if opponent == &Play::Rock {
						Outcome::Win
					} else {
						Outcome::Lose
					}
				},
				Play::Scissor => {
					if opponent == &Play::Paper {
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
}

#[derive(Debug)]
struct Match {
	pub you: Play,
	pub opponent: Play,
}

impl Match {
	pub fn from(s: &str) -> Self {
		Self {
			you: Play::from(s.chars().nth(2).unwrap()),
			opponent: Play::from(s.chars().next().unwrap()),
		}
	}

	pub fn outcome(&self) -> Outcome {
		Play::cmp(&self.you, &self.opponent)
	}

	pub fn score(&self) -> u8 {
		self.you.value() + self.outcome().score()
	}
}

fn matches() -> Vec<Match> {
	std::fs::read_to_string("res/aoc2022/day_2.txt").unwrap().lines().map(Match::from).collect()
}

pub fn rock_paper_scissors_part_1() -> u64 {
	matches()
		.iter()
		.map(|m| {
			println!("{m:#?} = {}", m.score());
			m.score() as u64
		})
		.sum()
}
