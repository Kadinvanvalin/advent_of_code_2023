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
            "467..11489".to_string(),
			"......*...".to_string(),
        ];
        
        let valid_digits = read_schematic(schematic);
        assert_eq!(count(valid_digits), 11489);
    }
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
        assert_eq!(count(valid_digits), 451490);
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
    let valid_digits: Vec<Digit> = read_schematic(schematic);
    
    println!("{}", count(valid_digits));
}
fn count(valid_digits: Vec<Digit>) -> i32 {
   let mut count = 0;
   println!("ALL VALID DIGIGITS COUNTING {:?}", valid_digits);
   for digit in &valid_digits {
        println!("valid digit:: {:?}", digit);
        count += digit.value.parse::<i32>().unwrap();
        println!("current count: {}", count);
    }
    return count;
}
fn read_schematic(schematic: Vec<String>) -> Vec<Digit> {
    let mut found_digits: Vec<Digit> = vec![];
    let mut valid_digits: Vec<Digit> = vec![];
    for (y, line) in schematic.clone().iter().enumerate() {
        let y: i32 = y.try_into().unwrap();
        let mut digit = Digit { y, start_x: -1, end_x: -1, value: String::new() };
        for (x, char) in line.chars().enumerate() {
        let x = x.try_into().unwrap();
            if char.is_digit(10) {
               if digit.start_x == -1 {
                    digit.start_x = x;
               } 
               digit.value.push(char);
                if x == <usize as TryInto<i32>>::try_into(line.len()).unwrap()  - 1 {

                 digit.end_x = x;
                 let complete_digit = Digit {
                    y,
                    start_x: digit.start_x,
                    end_x: digit.end_x,
                    value: digit.value,
                };
                    println!("complete_digit:: {:?}", complete_digit);
                found_digits.push(complete_digit);
                digit.start_x = -1;
                digit.end_x =  -1;
                digit.value = String::new();
                }
            } else {
               if !(digit.start_x == -1) {
                 digit.end_x = x;
                 let complete_digit = Digit {
                    y,
                    start_x: digit.start_x,
                    end_x: digit.end_x,
                    value: digit.value,
                };
                    println!("complete_digit:: {:?}", complete_digit);
                found_digits.push(complete_digit);
                digit.start_x = -1;
                digit.end_x =  -1;
                digit.value = String::new();
               }
            }
        }
        }
    
    println!("ALL DIGITS::: {:?}", found_digits);
     for digit in &found_digits {
         'outer: for x in digit.start_x..digit.end_x {
            println!("looking for digits:: x:: {}",x);
            let y = digit.y;
            for dir in DIRECTIONS {
            let (y_diff, x_diff) = dir;
                let y = y + y_diff; 
                let x = x + x_diff; 
            println!("looking for diff:: {}, {}", y, x);
                if y < 0 || x < 0 {
                    println!("out of bounds, skipping {}, {}", y,x);
                } else {
                println!("digit:: {:?}, x:: {}, y:: {}", digit, x, y);
                if y < schematic.len() as i32 {
                    let y = y as usize; 
                    let string = &schematic[y];
                    if x < string.len().try_into().unwrap() {
                        let char = &string.chars().nth(x.try_into().unwrap()).unwrap();
                                println!("looking for match for digit:: {:?} FOUND CHAR {}", digit, char);
                        if !char.is_digit(10) && char != &'.' {
                                println!("found match for digit:: {:?}", digit);
                             let complete_digit = Digit {
                                y: digit.y,
                                start_x: digit.start_x,
                                end_x: digit.end_x,
                                value: digit.value.clone(),
                            };
                           valid_digits.push(complete_digit);
                           break 'outer; 
                        }
                    }
                } 
                }
            }
        }
    }
    return valid_digits;
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
