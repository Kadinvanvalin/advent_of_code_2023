use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

struct Game {
    id: String,
    blue_count: u32,
    red_count: u32,
    green_count: u32,
}
fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(game) = line {
                let game = game.split(":");
                let id = game.clone()
                    .next()
                    .expect("should have a game id")
                    .split(" ")
                    .last()
                    .expect("Should have an ID for the game at this location");
                
                let rounds = game.last().expect("should have games data").split(&[';', ','][..])
                    .collect::<Vec<_>>();
                let mut largest_possible = Game {
                    id: id.to_string(),
                    blue_count: 0,    
                    red_count: 0,    
                    green_count: 0,    
                };

                for round in rounds {
                    if let Some(found_marbles) = round.trim().split_once(" ") {
                        
                        match found_marbles {
                            (count, "blue") => {
                                largest_possible.blue_count = count.parse::<u32>().unwrap();
                            },
                            (count, "red") => {
                                largest_possible.red_count = count.parse::<u32>().unwrap();
                            },
                            (count, "green") => {
                                largest_possible.green_count = count.parse::<u32>().unwrap();
                            },
                              (_, &_) => todo!(),

                        }
                    }
                };
                println!("game id: {}", id.to_string());
                println!("red count: {}", largest_possible.red_count);
                println!("blue count: {}", largest_possible.blue_count);
                println!("green count: {}", largest_possible.green_count);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
