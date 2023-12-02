use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

fn main() {
    println!("Hello, world!");
    if let Ok(lines) = read_lines("./input.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(code) = line {
                let chars = code.chars();
                let ints = find_two_ints(chars);
                count += ints;
                println!("{}:: {}:: count:: {}", code, ints, count);
            }
        }
    }
}
fn find_two_ints(chars: Chars<'_>) -> i32 {
    let mut x = chars
        .filter(|a| a.is_digit(10));
    let (first, last) = match (x.next(), x.last()) {
        (Some(a), Some(b)) => (a, b),
        (Some(a), None) => (a, a),
        (None, Some(b)) => (b, b),
        (None, None) => (' ', ' '),
    };
    format!("{}{}", first, last)
        .trim()
        .parse::<i32>()
        .unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
