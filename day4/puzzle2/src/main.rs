use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// TODO dynamic programming problem
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let card: Vec<String> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", // 4 points
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", // 2 points 0
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", // 3 points 6
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", // 1 points 2
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", // 1 points 1
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", // 0
        ]
        .iter()
        .map(|&s| s.into())
        .collect();
        assert_eq!(check_cards(card), 30);
    }
}

fn check_cards(cards: Vec<String>) -> i32 {
    let mut all_winners: Vec<Vec<i32>> = vec![];
    let mut all_have_numbers: Vec<Vec<i32>> = vec![];
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
        let have_numbers: Vec<i32> = card
            .clone()
            .last()
            .expect("should have have_numbers")
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        all_winners.push(winners.clone());
        all_have_numbers.push(have_numbers.clone());
        println!(
            "winners: {:?}, have_numbers: {:?}, points: {}",
            winners, have_numbers, points
        );
    }
    let mut total_points = 0;
    let mut points_map: HashMap<usize, i32> = HashMap::new();
    let points_map =
        all_have_numbers
            .iter()
            .enumerate()
            .fold(points_map, |acc, (i, have_numbers)| {
                let mut points = 0;
                for number in have_numbers {
                    if all_winners[i].contains(number) {
                        points += 1;
                    }
                }
                let mut return_value = acc.clone();
                return_value.insert(i, points);
                return_value
            });

    println!("PointsMap first pass:: {:?}", points_map);

    println!("PointsMap:: {:?}", points_map);
    let mut last_val = 0;
    let mut winner_winner_chicken_dinner: Vec<Vec<usize>> = vec![];
    for x in 0..(all_winners.len()) {
        let i = all_winners.len() - 1 - x;
        println!("i loop:: {}", i);
        let winners_on_card = points_map.get(&i).unwrap();
        let range = (*winners_on_card as usize) + i;
        winner_winner_chicken_dinner.push(vec![]);
        for get_another_card_of_this in i+1..range+1 {
            println!("is i mutating:: {}", x);
            winner_winner_chicken_dinner[x].push(get_another_card_of_this);
            println!("winner_winner:: {:?} adding i: {}", winner_winner_chicken_dinner, i);
        }
        println!("winner on card: {}", winners_on_card);
        println!("total_points: {}", total_points);
    }
    winner_winner_chicken_dinner.reverse();
    for winning_card in winner_winner_chicken_dinner.iter().rev() {
        let count = count_cards(&winner_winner_chicken_dinner.clone(), winning_card.clone().to_vec(), 0);
        println!("count:: {}", count);
        total_points += count;
        println!("total_points:: {}", total_points);
    }
    println!("winner_winner:: {:?}", winner_winner_chicken_dinner);
    println!("PointsMap:: {:?}", points_map);
    return total_points;
}

fn count_cards(winner_winner_chicken_dinner: &Vec<Vec<usize>>, mut winning_card: Vec<usize>, depth: i32) -> i32 {
    println!("(winner_winner_chicken_dinner: Vec<Vec<usize>>:: {:?}, mut winning_card: Vec<usize>:: {:?}, depth : {})", winner_winner_chicken_dinner, winning_card, depth);
    let mut return_value = 1;
    while let Some(new_card) = winning_card.pop() {
            println!("won new_cards:: {:?}", new_card); 
            let won_a_card = &winner_winner_chicken_dinner[new_card].clone();
            return_value += count_cards(&winner_winner_chicken_dinner, won_a_card.clone().to_vec(), depth + 1);
    };
    println!("returning for depth: {}, return_value: {}", depth, return_value);
    return return_value;
}

fn main() {
    let mut input = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(code) = line {
                input.push(code);
            }
        }
    }
    println!("{:?}", check_cards(input));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
