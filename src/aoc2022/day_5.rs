use std::{
	convert::Infallible,
	ops::RangeFrom,
	str::FromStr,
};

use derive_new::new;

#[derive(Debug, Default, new)]
struct Supplies {
	stacks: Vec<Vec<char>>,
}

impl Supplies {
	fn apply(&mut self, instruction: Instruction) {
		let mut stack_from = self.stacks[instruction.from].clone();
		let mut stack_to = self.stacks[instruction.to].clone();
		stack_to.append(
			&mut stack_from
				.drain::<RangeFrom<usize>>(stack_from.len() - instruction.amount as usize..)
				.rev()
				.take(instruction.amount.into())
				.collect(),
		);
		self.stacks[instruction.from] = stack_from;
		self.stacks[instruction.to] = stack_to;
	}

	fn top_level(&self) -> String {
		self.stacks.iter().map(|stack| stack.last().unwrap_or(&' ')).collect()
	}
}

impl ToString for Supplies {
	fn to_string(&self) -> String {
		self.stacks
			.iter()
			.map(|stack| stack.iter().map(char::to_string).collect::<Vec<_>>().join(""))
			.collect::<Vec<_>>()
			.join("\n")
	}
}

impl FromStr for Supplies {
	// just panic man
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let layers = s
			.lines()
			.rev()
			.map(str::to_string)
			.map(|line| {
				let mut crates = vec![];
				let mut index = 0;
				while index * 4 < line.len() {
					crates.push(&line[index * 4..index * 4 + 3]);
					index += 1;
				}
				crates
					.into_iter()
					.enumerate()
					.filter(|(_, s)| !s.contains(" "))
					.map(|(index, s)| (index, s.chars().take(2).collect::<Vec<_>>()[1]))
					.collect::<Vec<_>>()
			})
			.collect::<Vec<Vec<_>>>();

		let stack_len = layers
			.iter()
			.map(|layer| layer.iter().fold(0, |acc, (index, _)| *index.max(&acc)))
			.fold(0, |acc, index| index.max(acc))
			+ 1;
		let mut stacks = vec![Vec::new(); stack_len];

		for layer in layers {
			for (index, c) in layer {
				stacks[index].push(c);
			}
		}
		Ok(Supplies::new(stacks))
	}
}

#[derive(Debug, Clone, Copy, new)]
struct Instruction {
	amount: u8,
	from: usize,
	to: usize,
}

impl FromStr for Instruction {
	// just panic man
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut tokens = s.split(" ");
		let mut every_second_number = || {
			tokens.next();
			tokens.next().unwrap().parse::<usize>().unwrap()
		};
		Ok(Self::new(
			every_second_number() as u8,
			every_second_number() - 1,
			every_second_number() - 1,
		))
	}
}

fn input() -> (Supplies, Vec<Instruction>) {
	let input = std::fs::read_to_string("input/aoc2022/day_5.txt").unwrap();
	let lines = input.lines().collect::<Vec<_>>();
	let supplies = Supplies::from_str(lines[0..9].join("\n").as_str()).unwrap();
	let instructions =
		lines[10..].into_iter().map(|line| Instruction::from_str(line).unwrap()).collect();
	(supplies, instructions)
}

pub fn supply_stack_part_1() -> String {
	let (mut supplies, instructions) = input();
	for instruction in instructions {
		supplies.apply(instruction);
	}
	supplies.top_level()
}
