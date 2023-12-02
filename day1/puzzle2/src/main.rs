use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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

    #[test]
    fn test_oneone() {
        assert_eq!(parse("oneone".to_string()), "11".to_string());
    }
}
struct FoundDigit {
    index: usize,
    token: String,
}
fn parse(input: String) -> String {
    let mut result = String::new();
    let mut tokens = Vec::new();

    for (index, _) in input.match_indices("one").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "1".to_string()})
    };

    for (index, _) in input.match_indices("two").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "2".to_string()})
    };

    for (index, _) in input.match_indices("three").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "3".to_string()})
    };

    for (index, _) in input.match_indices("four").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "4".to_string()})
    };

    for (index, _) in input.match_indices("five").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "5".to_string()})
    };


    for (index, _) in input.match_indices("six").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "6".to_string()})
    };
    
    for (index, _) in input.match_indices("seven").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "7".to_string()})
    };

    for (index, _) in input.match_indices("eight").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "8".to_string()})
    };

    for (index, _) in input.match_indices("nine").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "9".to_string()})
    };

    for (index, _) in input.match_indices("1").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "1".to_string()})
    };

    for (index, _) in input.match_indices("2").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "2".to_string()})
    };

    for (index, _) in input.match_indices("3").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "3".to_string()})
    };

    for (index, _) in input.match_indices("4").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "4".to_string()})
    };

    for (index, _) in input.match_indices("5").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "5".to_string()})
    };


    for (index, _) in input.match_indices("6").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "6".to_string()})
    };
    
    for (index, _) in input.match_indices("7").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "7".to_string()})
    };

    for (index, _) in input.match_indices("8").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "8".to_string()})
    };

    for (index, _) in input.match_indices("9").collect::<Vec<_>>() {
        tokens.push(FoundDigit { index, token: "9".to_string()})
    };
    tokens.sort_by(|a, b| a.index.cmp(&b.index));
    for token in tokens { // instead of iterating you should do this recursivly and reduce the
        // string each time  
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
                let ints = find_two_ints(code.clone());
                count += ints;
                println!("{}:: {}:: count:: {}", code, ints, count);
            }
        }
    }
}
fn find_two_ints(string: String) -> i32 {
    let binding = parse(string);
    let mut x = binding.chars();
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
