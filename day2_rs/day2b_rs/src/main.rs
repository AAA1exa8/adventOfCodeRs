use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let input = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day2_rs/day2b_rs/src/inputs/input.txt";
    let input2 = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day2_rs/day2b_rs/src/inputs/input2.txt";
    let file = File::open(input).expect("Failed to open the file");
    let reader = BufReader::new(file);
    #[derive(Clone, Copy)]
    enum State{
        LOSE,
        DRAW,
        WIN
    }
    let mut elf_1 = 'A';
    let mut ex_state = 'X';
    let mut score = 0;

    const ROCK: [i32; 3] = [3, 1, 2];
    const PAPER: [i32; 3] = [1, 2, 3];
    const SCISSOR: [i32; 3] = [2, 3, 1]; 

    for line in reader.lines(){
        let line = line.expect("failed to load line");
        elf_1 = line.chars().nth(0).unwrap();
        ex_state = line.chars().nth(2).unwrap();
        match ex_state {
            'X' => swindle(&State::LOSE, &mut score, elf_1),
            'Y' => swindle(&State::DRAW, &mut score, elf_1),
            'Z' => swindle(&State::WIN, &mut score, elf_1),
            _ => print!("you fucked up")
            
        }

    }
    print!("{}", score);

    fn swindle(state: &State, scorey: &mut i32, elf: char){
        *scorey += (*state as i32)*3;
        match elf{
            'A' => *scorey += ROCK[*state as usize],
            'B' => *scorey += PAPER[*state as usize],
            'C' => *scorey += SCISSOR[*state as usize],
            _ => print!("you fucked up")
        }
    }
}