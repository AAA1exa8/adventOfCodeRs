use std::{fs::File, io::{BufRead, BufReader}, vec};

fn main() {
    let input = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day1_rs/day1a_rs/src/inputs/input.txt";
    let input2 = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day1_rs/day1a_rs/src/inputs/input2.txt";
    let file = File::open(input).expect("Failed to pen the file");
    let reader = BufReader::new(file);

    let mut current_elf_value = 0;
    let mut highest_elf_value = vec![0,0,0];

    for line in reader.lines(){
        let line = line.expect("failed to load line");
        if line != "" {
            current_elf_value += line.trim().parse::<i32>().expect("nan");
            continue;
        }
        compare( &current_elf_value, &mut highest_elf_value);
        highest_elf_value.sort();
        current_elf_value = 0;  
    }
    compare( &current_elf_value, &mut highest_elf_value);
    print!("{},{:?} ",highest_elf_value.iter().sum::<i32>() ,highest_elf_value);

    fn compare(current_ev: &i32, highest_ev:&mut Vec<i32>){
        for i in 0..3{
            if current_ev > &highest_ev[i]{
                highest_ev[i] = *current_ev;
                return;
            }
        }
    }
}