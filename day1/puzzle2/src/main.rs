use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(parse("one".to_string()), "1".to_string());
    }

    #[test]
    fn test_ooe() {
        assert_eq!(parse("ooe".to_string()), " ".to_string());
    }

    #[test]
    fn test_two() {
        assert_eq!(parse("two".to_string()), "2".to_string());
    }

    #[test]
    fn test_onetwo() {
        assert_eq!(parse("onetwo".to_string()), "12".to_string());
    }

    #[test]
    fn test_twone() {
        assert_eq!(parse("twone".to_string()), "21".to_string());
    }
}
struct FoundDigit {
    index: usize,
    token: String,
}
fn parse(input: String) -> String {
    let mut result = String::new();
    let mut tokens = Vec::new();
    if let Some(one) = input.find("one") {
        tokens.push(FoundDigit { index: one, token: "1".to_string()})
    }

    if let Some(two) = input.find("two") {
        tokens.push(FoundDigit { index: two, token: "2".to_string()})
    }
    tokens.sort_by(|a, b| a.index.cmp(&b.index));
    for token in tokens {
        result.push_str(&token.token);
    }
    if result.is_empty() {
        result.push_str(" ");
    }
        // grab whichever is lowest, and start new string after first index?
    return result;
}

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
