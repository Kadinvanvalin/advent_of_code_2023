use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

struct Game {
    id: u32,
    blue_count: u32,
    red_count: u32,
    green_count: u32,
}
fn main() {
            let mut added_count: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(game) = line {
            let debug = game.clone();
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
                    id: id.parse::<u32>().expect("Id should be a u32"),
                    blue_count: 0,    
                    red_count: 0,    
                    green_count: 0,    
                };

                for round in rounds {
                    if let Some(found_marbles) = round.trim().split_once(" ") {
                        
                        match found_marbles {
                            (count, "blue") => {
                                let count = count.parse::<u32>().unwrap();
                                if count > largest_possible.blue_count {
                                    largest_possible.blue_count = count;
                                }
                            },
                            (count, "red") => {
                                let count = count.parse::<u32>().unwrap();
                                if count > largest_possible.red_count {
                                    largest_possible.red_count = count;
                                }
                            },
                            (count, "green") => {
                                let count = count.parse::<u32>().unwrap();
                                if count > largest_possible.green_count {
                                    largest_possible.green_count = count;
                                }
                            },
                            (_, &_) => todo!(),

                        }
                    }
                };
            // 12 red cubes, 13 green cubes, and 14 blue cubes
            if largest_possible.id == 1 {
                println!("raw game: {}", debug);
                println!("game id: {}", largest_possible.id);
                println!("red count: {}", largest_possible.red_count);
                println!("blue count: {}", largest_possible.blue_count);
                println!("green count: {}", largest_possible.green_count);
             }
                added_count += largest_possible.red_count * largest_possible.blue_count * largest_possible.green_count;
           //     println!("game id: {}", largest_possible.id);
           //     println!("red count: {}", largest_possible.red_count);
           //     println!("blue count: {}", largest_possible.blue_count);
           //     println!("green count: {}", largest_possible.green_count);
            }
        }
    }
                println!("count: {}", added_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
