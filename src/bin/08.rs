use fxhash::FxHashSet;
use std::str::FromStr;
advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Nop(isize),
    Jmp(isize),
    Acc(i32),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ").ok_or(ParseInstructionError)? {
            ("nop", v) => Ok(Instruction::Nop(
                v.parse().map_err(|_| ParseInstructionError)?,
            )),
            ("acc", v) => Ok(Instruction::Acc(
                v.parse().map_err(|_| ParseInstructionError)?,
            )),
            ("jmp", v) => Ok(Instruction::Jmp(
                v.parse().map_err(|_| ParseInstructionError)?,
            )),
            _ => Err(ParseInstructionError),
        }
    }
}

enum ProgramExecutionResult {
    Infinite(u32),
    Ended(u32),
}

fn execute_program(program: &Vec<Instruction>) -> ProgramExecutionResult {
    let mut pc = 0usize;
    let mut acc = 0;
    let mut visited = FxHashSet::default();

    while !visited.contains(&pc) {
        visited.insert(pc);

        match program.get(pc) {
            Some(instruction) => {
                (pc, acc) = match instruction {
                    Instruction::Nop(_) => (pc + 1, acc),
                    Instruction::Jmp(n) => (((pc as isize) + *n) as usize, acc),
                    Instruction::Acc(n) => (pc + 1, acc + n),
                }
            }
            None => return ProgramExecutionResult::Ended(acc as u32),
        }
    }

    ProgramExecutionResult::Infinite(acc as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let program: Vec<Instruction> = input
        .lines()
        .map(|instruction| instruction.parse().unwrap())
        .collect();

    match execute_program(&program) {
        ProgramExecutionResult::Infinite(n) => Some(n),
        ProgramExecutionResult::Ended(_) => panic!("Program ended"),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut program: Vec<Instruction> = input
        .lines()
        .map(|instruction| instruction.parse().unwrap())
        .collect();

    for i in 0..program.len() {
        let instruction = program.get_mut(i).unwrap();
        let tmp: Instruction = *instruction;

        match instruction {
            Instruction::Nop(n) => *instruction = Instruction::Jmp(*n),
            Instruction::Jmp(n) => *instruction = Instruction::Nop(*n),
            Instruction::Acc(_) => continue,
        }

        match execute_program(&program) {
            ProgramExecutionResult::Infinite(_) => {}
            ProgramExecutionResult::Ended(n) => return Some(n),
        }

        let instruction = program.get_mut(i).unwrap();
        *instruction = tmp;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
