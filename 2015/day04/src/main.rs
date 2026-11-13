use std::error::Error;
use std::fs;

struct Solution {
    part1: i32,
    part2: i32,
}

fn search_md5(input: &String, leading: i32) -> i32 {
    let mut i: i32 = 0;
    loop {
        let guess = format!("{0}{1}", input, i);
        let hash = format!("{:x}", md5::compute(guess));

        if hash.starts_with(&"0".repeat(leading as usize)) {
            return i;
        }
        i += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("./src/input.txt")?;
    let input = file.lines().collect();
    // let results = "ckczppom".to_string();
    let mut solution = Solution { part1: 0, part2: 0 };

    println!("'{}'", input);

    solution.part1 = search_md5(&input, 5);
    println!("Part 1 Solution: {}", solution.part1);

    solution.part2 = search_md5(&input, 6);
    println!("Part 2 Solution: {}", solution.part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let answer = search_md5(&"abcdef".to_string(), 5);
        assert_eq!(609043, answer)
    }

    #[test]
    fn example_2() {
        let answer = search_md5(&"pqrstuv".to_string(), 5);
        assert_eq!(1048970, answer)
    }
}
