use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_two() {
        let card: Vec<String> = vec![
			"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
			"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
			"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
			"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
			"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
			"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ].iter().map(|&s|s.into()).collect();
        assert_eq!(check_cards(card), 13);
    }
}
fn check_cards(card: Vec<String>) -> i32 {
    return 12;
}
fn main() {
    let mut schematic = Vec::new();
    if let Ok(lines) = read_lines( "./input.txt") {
        for line in lines {
            if let Ok(code) = line {
                schematic.push(code);
            }
        }
    }
    println!("{:?}", schematic);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}