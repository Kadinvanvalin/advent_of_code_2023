use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
//    #[test]
//    fn test_one() {
//       let schematic: Vec<String> = vec![
//            "467...5...".to_string(),
//            "*.....*...".to_string(),
//            ".....10..9".to_string(),
//        ];
//        
//        let numbers = read_schematic(schematic);
//        let count = count(numbers);
//        assert_eq!(count, 50);
//    }
    #[test]
    fn test_two() {
       let schematic: Vec<String> = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        
        let valid_digits = read_schematic(schematic);
        assert_eq!(count(valid_digits), 467835);
    }
}
    const DIRECTIONS: &'static [(i32, i32)] = &[
        (-1,-1),
        (-1,0),
        (-1,1),
        (0,-1),
        (0,1),
        (1,-1),
        (1,0),
        (1,1),
    ];
#[derive(Debug)]
#[derive(Clone)]
struct Digit {
    y: i32,
    x: i32,
    value: String,
}

#[derive(Debug)]
#[derive(Clone)]
struct Token {
    y: i32,
    x: i32,
    digits: Vec<Digit>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Numbers {
    numbers: Vec<Number>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Number {
    y: i32,
    start_x: i32,
    end_x: i32,
    value: String,
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
    let numbers = read_schematic(schematic);
    let count = count(numbers.clone());
    println!("{:?}", numbers);
    println!("{:?}", count);
}

fn count(tokens_numbers: Vec<Numbers>) -> i32 {
    println!("{:?}", tokens_numbers);
    let mut count = 0;
    for numbers in tokens_numbers {
        println!("numbers in token_numbers : {:?}" , numbers);
        if numbers.numbers.len() > 1 {
        
        let mut token_count = 1;
        for number in numbers.numbers {
            println!("adding numbers: {:?}", number);
            let num: i32 = number.value.parse().unwrap();
            token_count *= num;
        }
        
        count += token_count;
        }
    }
    return count
}

fn read_schematic(schematic: Vec<String>) -> Vec<Numbers> {
    let mut found_tokens: Vec<Token> = vec![];
    for (y, line) in schematic.clone().iter().enumerate() {
        let y: i32 = y.try_into().unwrap();
        // let mut digit = Digit { y, start_x: -1, end_x: -1, value: String::new() };
        for (x, char) in line.chars().enumerate() {
        let x = x.try_into().unwrap();
            if char == '*' {
                found_tokens.push(Token { y, x, digits: vec![] });
               }
            }
        }
        
    println!("looking for digits: {:?}", found_tokens);
     let mut tokens_numbers: Vec<Numbers> = vec![]; 
     for token in &mut found_tokens {
            let mut numbers = Numbers {numbers: vec![]}; 
            let y = token.y;
            let x = token.x;
            for dir in DIRECTIONS {
            let (y_diff, x_diff) = dir;
                let y = y + y_diff; 
                let x = x + x_diff; 
            println!("looking for diff:: {}, {}", y, x);
                if y < 0 || x < 0 {
                    println!("out of bounds, skipping {}, {}", y,x);
                } else {
                //println!("digit:: {:?}, x:: {}, y:: {}", digit, x, y);
                if y < schematic.len() as i32 {
                    let y = y as usize; 
                    let string = &schematic[y];
                    if x < string.len().try_into().unwrap() {
                        let char = &string.chars().nth(x.try_into().unwrap()).unwrap();
                        if char.is_digit(10) {
                        if y  < schematic.len() {
                            let mut i = x + 1;
                            let mut j = x - 1;
                            let mut number = Number {
                                y: y as i32,
                                start_x: x,
                                end_x: x,
                                value: String::new(),
                            };
                                println!("creating number for token: {:?}", number);
                                number.value.push(*char);
                            while i < string.len() as i32 {
                                let char = &string.chars().nth(i.try_into().unwrap()).unwrap();
                                
                                if char.is_digit(10) {
                                        number.value.push(*char);
                                            number.end_x = i;
                                println!("pushing number for token: {:?}, x: {}", number, i);
                                    } else { break; }
                                i = i + 1;
                            }
                            while j >= 0 {
                                let char = &string.chars().nth(j.try_into().unwrap()).unwrap();
                                if char.is_digit(10) {
                                            number.value = char.to_string() + & number.value;
                                            number.start_x = j;
                                println!("pushing number for token: {:?}, x: {}", number, j);
                                    } else { break; }
                                j = j - 1;
                            }
                                println!("pushing number for token: {:?}", number);
                                numbers.numbers.push(number);
                        }
                    }
                } 
            }
            }        
        }
        tokens_numbers.push(numbers);
    }
    // dedup
    //
    let mut deduped_numbers: Vec<Numbers> = vec![]; 
    let mut dedup: Vec<HashMap<String, Number>> = vec![];
    for (i, _) in tokens_numbers.iter().enumerate() {
        dedup.push(HashMap::new());
    }
    for (i, el) in tokens_numbers.iter().enumerate() {
        for num in &el.numbers {
            let key = num.start_x.to_string() + &num.end_x.to_string() + &num.y.to_string();
            dedup[i].insert(key.to_string(), num.clone());
        }
    }
    for (i, hash) in dedup.iter().enumerate() {
        let numbers =  Numbers {numbers: vec![]};
        deduped_numbers.push(numbers);
        for (key, number) in hash.into_iter() {
            deduped_numbers[i].numbers.push(number.clone());
            println!("key: {} number: {:?}", key, number)         
        }
    }
    return deduped_numbers;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
