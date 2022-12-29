use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let input = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day1_rs/day1a_rs/src/inputs/input.txt";
    let input2 = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day1_rs/day1a_rs/src/inputs/input2.txt";
    let file = File::open(input).expect("Failed to pen the file");
    let reader = BufReader::new(file);

    let mut current_elf = 1;
    let mut current_elf_value = 0;
    let mut highest_elf = 0;
    let mut highest_elf_value = 0;

    for line in reader.lines(){
        let line = line.expect("failed to load line");
        if line != "" {
            current_elf_value += line.trim().parse::<i32>().expect("nan");
            continue;
        }
        compare(&current_elf, &current_elf_value, &mut highest_elf, &mut highest_elf_value);
        current_elf += 1;
        current_elf_value = 0;  
    }
    compare(&current_elf, &current_elf_value, &mut highest_elf, &mut highest_elf_value);
    print!("{}, {}: ", highest_elf, highest_elf_value);

    fn compare(current_e: &i32, current_ev: &i32, highest_e: &mut i32, highest_ev:&mut i32){
        if current_ev > highest_ev{
            *highest_e = *current_e;
            *highest_ev = *current_ev;
        }
    }
}

