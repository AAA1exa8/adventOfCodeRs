use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let input = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day2_rs/day2a_rs/src/inputs/input.txt";
    let input2 = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day2_rs/day2a_rs/src/inputs/input2.txt";
    let file = File::open(input).expect("Failed to open the file");
    let reader = BufReader::new(file);

    let mut elf_1 = 'A';
    let mut elf_y = 'X';
    let mut score = 0;

    for line in reader.lines(){
        let line = line.expect("failed to load line");
        elf_1 = line.chars().nth(0).unwrap();
        elf_y = line.chars().nth(2).unwrap();
        match elf_y {
            'X' => score += 1,
            'Y' => score += 2,
            'Z' => score += 3,
            _  => print!("you fucked up")
        }
        match vec![&elf_y, &elf_1][..]{
            ['X', 'A'] | ['Y', 'B'] | ['Z', 'C'] => score += 3,
            ['X', 'C'] | ['Y', 'A'] | ['Z', 'B'] => score +=6,
            _=> score += 0
        }
    }
    print!("{}", score);
}