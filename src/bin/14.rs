use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

advent_of_code::solution!(14);

#[derive(PartialEq, Eq, Copy, Clone)]

// (positive, negative)
struct Mask(u64, u64);

impl Mask {
    pub fn apply(&self, x: u64) -> u64 {
        (x | self.0) & !self.1
    }
}

impl Debug for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:064b} -{:064b}", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    SetMask(Mask),       // (positive, negative)
    SetMemory(u64, u64), // (address, value)
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"mask = (.{36})").unwrap();

        if let Some(caps) = re.captures(s) {
            let (positive, negative) =
                caps[1]
                    .chars()
                    .rev()
                    .enumerate()
                    .fold((0, 0), |(positive, negative), (i, c)| {
                        match c {
                            '1' => (positive + (1u64 << i), negative),
                            '0' => (positive, negative + (1u64 << i)),
                            _ => (positive, negative),
                        }
                    });
            return Ok(Instruction::SetMask(Mask(positive, negative)));
        }

        let re = Regex::new(r"mem\[(.+)] = (.+)").unwrap();

        if let Some(caps) = re.captures(s) {
            return Ok(Instruction::SetMemory(
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
            ));
        }

        Err(ParseInstructionError)
    }
}

#[derive(Debug)]
struct ProgramState {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial_state = ProgramState {
        mask: Mask(0, 0),
        memory: HashMap::new(),
    };
    let state: ProgramState = input
        .lines()
        .map(|instruction| instruction.parse().unwrap())
        .fold(initial_state, |mut state, instruction| {
            match instruction {
                Instruction::SetMask(mask) => {state.mask = mask},
                Instruction::SetMemory(address, value) => {
                    state.memory.insert(address, state.mask.apply(value));
                }
                _ => panic!("unreachable"),
            }
            state
        });

    Some(state.memory.values().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        let positive = 0b100010u64;
        let negative = 0b000100u64;
        let value = 0b001111u64;
        let expected = 0b101011u64;

        assert_eq!(Mask(positive, negative).apply(value), expected)
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
