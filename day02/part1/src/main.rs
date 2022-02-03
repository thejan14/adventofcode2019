use std::fs;
use std::path::Path;
use std::time::Instant;

enum InstructionType {
    Addition = 1,
    Multiplication = 2, 
}

fn main() {
    let data = fs::read_to_string(Path::new("../input.txt")).unwrap();
    let now = Instant::now();

    let mut programm = data
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    programm[1] = 12;
    programm[2] = 2;

    let mut pos = 0;
    loop {
        match programm[pos] {
            1 => exectute_instruction(&mut programm, pos, InstructionType::Addition),
            2 => exectute_instruction(&mut programm, pos, InstructionType::Multiplication),
            _ => break,
        }

        pos += 4;
    }

    let result = programm[0];

    let elapsed = now.elapsed();
    println!("{result} {elapsed:.2?}");
}

fn exectute_instruction(programm: &mut Vec<usize>, pos: usize, instruction: InstructionType){
    if programm.len() > pos + 2 {
        let lefthand_pos = programm[pos + 1];
        let righthand_pos = programm[pos + 2];
        let target_pos = programm[pos + 3];
        match instruction {
            InstructionType::Addition => programm[target_pos] = programm[lefthand_pos] + programm[righthand_pos],
            _ => programm[target_pos] = programm[lefthand_pos] * programm[righthand_pos],
        }
    }
}
