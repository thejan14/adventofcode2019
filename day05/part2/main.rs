use std::fs;
use std::path::Path;
use std::time::Instant;

enum InstructionType {
    Addition = 1,
    Multiplication = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Exit = 99,
}

#[derive(Clone)]
enum ParamMode {
    Position = 0,
    Immediate = 1,
}

fn main() {
    let data = fs::read_to_string(Path::new("day05/input.txt")).unwrap();
    let now = Instant::now();

    let memory = data
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut memory_clone = memory.clone();

    let result = run_intcode(&mut memory_clone, 5);

    let elapsed = now.elapsed();
    println!("{result} {elapsed:.2?}");
}

fn run_intcode(memory: &mut Vec<i32>, input: i32) -> i32 {
    let mut output = 0;
    let mut instruction_pointer = 0;
    let mut exited = false;
    while !exited {
        let instruction_code: Vec<u32> = memory[instruction_pointer]
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();

        let instruction_type = get_instruction_type(&instruction_code);

        let param_count = get_param_count(&instruction_type);
        let param_mode_keys =
            &instruction_code[0..instruction_code.len().checked_sub(2).map_or(0, |x| x)];
        let virtual_zeros = param_count
            .checked_sub(param_mode_keys.len())
            .map_or(0, |x| x);
        let param_values: Vec<i32> = (0..param_count)
            .map(|i| {
                i.checked_sub(virtual_zeros)
                    .map_or(ParamMode::Position, |j| match param_mode_keys[j] {
                        0 => ParamMode::Position,
                        _ => ParamMode::Immediate,
                    })
            })
            .enumerate()
            .map(|(i, mode)| {
                // as parameter modes are in reverse order we always start with the position argument first when enumerating
                let pos = instruction_pointer + param_count - i;
                let value = memory[pos];
                match i {
                    0 if has_position_param(&instruction_type) => value,
                    _ => match mode {
                        ParamMode::Position => memory[usize::try_from(value).unwrap()],
                        ParamMode::Immediate => value,
                    },
                }
            })
            .rev() // revert params to original order
            .collect();

        let mut jump = false;
        let target = usize::try_from(param_values.last().map_or(0, |x| *x)).unwrap();
        match instruction_type {
            InstructionType::Addition => memory[target] = param_values[0] + param_values[1],
            InstructionType::Multiplication => memory[target] = param_values[0] * param_values[1],
            InstructionType::Input => memory[target] = input,
            InstructionType::Output => output = param_values[0],
            InstructionType::JumpIfTrue => {
                if param_values[0] != 0 {
                    instruction_pointer = usize::try_from(param_values[1]).unwrap();
                    jump = true;
                }
            }
            InstructionType::JumpIfFalse => {
                if param_values[0] == 0 {
                    instruction_pointer = usize::try_from(param_values[1]).unwrap();
                    jump = true;
                }
            }
            InstructionType::LessThan => {
                memory[target] = (param_values[0] < param_values[1]) as i32
            }
            InstructionType::Equals => memory[target] = (param_values[0] == param_values[1]) as i32,
            InstructionType::Exit => exited = true,
        }

        if !jump {
            instruction_pointer += param_values.len() + 1;
        }
    }

    return output;
}

fn get_param_count(instruction_type: &InstructionType) -> usize {
    return match instruction_type {
        InstructionType::Addition => 3,
        InstructionType::Multiplication => 3,
        InstructionType::Input => 1,
        InstructionType::Output => 1,
        InstructionType::JumpIfTrue => 2,
        InstructionType::JumpIfFalse => 2,
        InstructionType::LessThan => 3,
        InstructionType::Equals => 3,
        InstructionType::Exit => 0,
    };
}

fn get_instruction_type(instruction_code: &Vec<u32>) -> InstructionType {
    if let [.., key] = instruction_code[..] {
        return match key {
            1 => InstructionType::Addition,
            2 => InstructionType::Multiplication,
            3 => InstructionType::Input,
            4 => InstructionType::Output,
            5 => InstructionType::JumpIfTrue,
            6 => InstructionType::JumpIfFalse,
            7 => InstructionType::LessThan,
            8 => InstructionType::Equals,
            _ => InstructionType::Exit,
        };
    } else {
        panic!();
    }
}

fn has_position_param(instruction_type: &InstructionType) -> bool {
    return match *instruction_type {
        InstructionType::Output => false,
        InstructionType::JumpIfTrue => false,
        InstructionType::JumpIfFalse => false,
        _ => true,
    };
}
