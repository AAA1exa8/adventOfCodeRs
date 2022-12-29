use std::{fs::File, collections::HashSet, io::{BufReader, BufRead}};
fn main() {
    let input = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day3_rs/day3a_rs/src/inputs/input.txt";
    let input2 = "/home/plexandr/Documents/programming/RustCProjects/adventOfCodeRs/day3_rs/day3a_rs/src/inputs/input2.txt";
    let file = File::open(input).expect("Failed to open the file");
    let reader = BufReader::new(file);
    
    
    static ALPHABET: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut num: u32 = 0;
    
    for line in reader.lines(){
        let line = line.expect("cannot load line").chars().collect::<Vec<char>>();
        // let rux_1 = &line[..((line.len()+1)/2)];
        // let rux_2 = &line[((line.len()+1)/2)..];
        let rux_1:HashSet<&char> = line[..((line.len()+1)/2)].into_iter().collect();
        let rux_2: HashSet<&char> = line[((line.len()+1)/2)..].into_iter().collect();
        let intersect = rux_1.intersection(&rux_2).clone();
        let foo: char = **intersect.into_iter().next().unwrap();
        num += ALPHABET.iter().position(|x| *x == foo).unwrap() as u32 +1;
    }
    print!("{}", num)
}
