use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let input = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day3_rs/day3a_rs/src/inputs/input.txt";
    let input2 = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day3_rs/day3a_rs/src/inputs/input2.txt";
    let file = File::open(input).expect("Failed to open the file");
    let reader = BufReader::new(file);

    static ALPHABET: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut num: u32 = 0;
    let mut iterator: u8 = 0;
    let mut rux_1: HashSet<char> = HashSet::new();
    let mut rux_2: HashSet<char> = HashSet::new();
    let mut rux_3: HashSet<char> = HashSet::new();
    
    for line in reader.lines(){
        let line = line.expect("cannot load line").chars().collect::<Vec<char>>();
        match iterator {
            0 => {
                rux_1 = line[..].into_iter().cloned().collect();
                iterator += 1;},
            1 => {
                rux_2 = line[..].into_iter().cloned().collect();
                iterator += 1;}, 
            2 => {
                iterator = 0;
                rux_3 = line[..].into_iter().cloned().collect(); 
                let intersection: HashSet<char> = rux_1.intersection(&rux_2).cloned().collect();
                let intersection: HashSet<char> = intersection.intersection(&rux_3).cloned().collect();
                let character: char = intersection.into_iter().next().unwrap();
                num += ALPHABET.iter().position(|x| *x == character).unwrap() as u32 +1;
            },
            _ => println!("you fucked up")
        }
    }
    print!("{}", num);
}
