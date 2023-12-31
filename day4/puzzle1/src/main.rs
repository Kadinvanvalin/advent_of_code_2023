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

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<String>,
    numbers: Vec<String>,
}

fn check_cards(cards: Vec<String>) -> i32 {
    let mut total_points = 0;
    for card in cards {
        let mut points: i32 = 0;
        let card = card.split('|');
        let winners: Vec<i32> = card
            .clone()
            .next()
            .expect("should have winners")
            .split(":")
            .last()
            .expect("should have numbers")
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut win_map: HashMap<i32, i32> = HashMap::new();
        for num in &winners {
            win_map.insert(*num, 1);
        }
        let have_numbers: Vec<i32> = card
            .clone()
            .last()
            
            .expect("should have have_numbers")
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect(); 
       for number in &have_numbers {
           if winners.contains(&number) {
                if points == 0 {
                    points = 1;
                } else {
                    points = points * 2;
                }
                println!("found a winner!: {}:: new points {}", number, points);
            }
        }
        total_points += points;
    println!("winners: {:?}, have_numbers: {:?}, points: {}", winners, have_numbers, points);
    }
    return total_points;
}

fn main() {
    let mut input = Vec::new();
    if let Ok(lines) = read_lines( "./input.txt") {
        for line in lines {
            if let Ok(code) = line {
                input.push(code);
            }
        }
    }
    println!("{:?}", check_cards(input));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
