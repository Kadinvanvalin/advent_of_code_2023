use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_one() {
       let schematic: Vec<String> = vec![
            "467...5...".to_string(),
            "*.....*...".to_string(),
            ".....10..9".to_string(),
        ];
        
        let found_tokens = read_schematic(schematic);
   let numbers = flatten(found_tokens.clone());
    let count = count(numbers);
        assert_eq!(count, 50);
    }
//    #[test]
//    fn test_two() {
//       let schematic: Vec<String> = vec![
//            "467..114..".to_string(),
//            "...*......".to_string(),
//            "..35..633.".to_string(),
//            "......#...".to_string(),
//            "617*......".to_string(),
//            ".....+.58.".to_string(),
//            "..592.....".to_string(),
//            "......755.".to_string(),
//            "...$.*....".to_string(),
//            ".664.598..".to_string(),
//        ];
//        
//        let valid_digits = read_schematic(schematic);
//        assert_eq!(count(valid_digits), 451490);
//    }
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
    let found_tokens: Vec<Token> = read_schematic(schematic);
   let numbers = flatten(found_tokens.clone());
    let count = count(numbers);
    println!("{:?}", found_tokens);
    println!("{:?}", count);
}
fn count (numbers: Vec<Number>) -> i32 {
    let mut count = 0;
    for number in numbers {
        let num: i32 = number.value.parse().unwrap();
        count += num;
    }
    return count
}
fn flatten(found_tokens: Vec<Token>) -> Vec<Number> {
   let mut flattened_numbers: Vec<Number> = vec![];
    let mut number: Number = Number {
       y: -1,
        start_x: -1,
        end_x: -1,
        value: String::new(),
    };
   for token in &found_tokens {
        for digit in &token.digits {
        if number.y == -1 {
                number.y = digit.y;
                number.start_x = digit.x;
                number.end_x = digit.x;
                number.value.push_str(&digit.value);
        }
        if digit.x + 1 == number.end_x {
                number.end_x = digit.x;
                number.value.push_str(&digit.value);
         }
        flattened_numbers.push(number.clone());
        number = Number {
       y: -1,
        start_x: -1,
        end_x: -1,
        value: String::new(),
    };
    }
   }
    return flattened_numbers;
}
fn read_schematic(schematic: Vec<String>) -> Vec<Token> {
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
        
    
     for token in &mut found_tokens {
         for x in token.x..token.x {
            println!("looking for digits:: x:: {}",x);
            let y = token.y;
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
                             let digit = Digit {
                                y: y as i32,
                                x,
                                value: char.to_string(),
                            };
                           token.digits.push(digit);
                        }
                    }
                } 
                }
            }
        
        }
    }
    return found_tokens;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
