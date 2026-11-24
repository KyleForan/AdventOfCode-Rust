use std::fs;

struct Solution {
    part1: i32,
    part2: i32,
}

fn encode_count(line: &str) -> i32 {

    let slash_count = line.chars().filter(|el| *el == '\\').count(); 
    let quote_count = line.chars().filter(|el| *el == '"').count(); 

    ( slash_count + quote_count ) as i32 + 2
}

fn decode_count(line: &str) -> i32 {
    let mut chars = line.chars();
    let mut count = 0;

    while let Some(chr) = chars.next() {
        if chr == '\\' {
            match chars.next() {
                Some('"' | '\\') => {}
                Some('x') => {
                    chars.next();
                    chars.next();
                }
                _ => {}
            }
        };
        count += 1;
    }

    count - 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("./src/input.txt")?;
    let input = file.lines();
    let mut solution = Solution { part1: 0, part2: 0 };

    for line in input {
        solution.part1 += line.len() as i32 - decode_count(line);
        solution.part2 += encode_count(line);
    }

    println!("{}", solution.part1);
    println!("{}", solution.part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let file: String = fs::read_to_string("./src/sample.txt").unwrap();
        let lines = file.lines();
        let mut total = 0;

        for line in lines {
            dbg!(line, decode_count(line));
            total += line.len() as i32 - decode_count(line);
        }

        dbg!(total);
        assert_eq!(total, 12);
    }

    #[test]
    fn test_example_2() {
        let file: String = fs::read_to_string("./src/sample.txt").unwrap();
        let lines = file.lines();
        let mut total = 0;

        for line in lines {
            dbg!(line, encode_count(line));
            total += encode_count(line);
        }

        assert_eq!(total, 19);
    }
}
