use std::fs;
use std::path::Path;
use std::time::Instant;

enum Instruction {
    Addition = 1,
    Multiplication = 2, 
}

fn main() {
    let data = fs::read_to_string(Path::new("../input.txt")).unwrap();
    let now = Instant::now();

    let memory = data
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let (noun, verb) = find_pair(&memory);

    let result = 100 * noun + verb;

    let elapsed = now.elapsed();
    println!("{result} {elapsed:.2?}");
}

fn find_pair(memory: &Vec<usize>) -> (usize, usize) {
    for noun in 0..=99usize {
        for verb in 0..=99usize {
            let mut memory_clone = memory.clone();
            memory_clone[1] = noun;
            memory_clone[2] = verb;
            let result = run_intcode(&mut memory_clone);
            if result == 19690720 {
                return (noun, verb);
            }
        }
    }

    (0, 0)
}

fn run_intcode(memory: &mut Vec<usize>) -> usize {
    let mut address = 0;
    loop {
        match memory[address] {
            1 => exectute_instruction(memory, address, Instruction::Addition),
            2 => exectute_instruction(memory, address, Instruction::Multiplication),
            _ => break,
        }

        address += 4;
    }

    memory[0]
}

fn exectute_instruction(memory: &mut Vec<usize>, address: usize, instruction: Instruction){
    if memory.len() > address + 2 {
        let lefthand_address = memory[address + 1];
        let righthand_address = memory[address + 2];
        let target_address = memory[address + 3];
        match instruction {
            Instruction::Addition => memory[target_address] = memory[lefthand_address] + memory[righthand_address],
            _ => memory[target_address] = memory[lefthand_address] * memory[righthand_address],
        }
    }
}
