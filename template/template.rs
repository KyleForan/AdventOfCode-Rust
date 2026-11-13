use std::fs;

struct Solution {
    part1:i32,
    part2:i32
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file = fs::read_to_string("./src/input.txt")?;
    let input = file.lines();
    let mut solution = Solution { part1: 0, part2: 0 };

    for line in input {
        solution.part1 += 0;
        solution.part2 += 0;
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
        assert_eq!(1,1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(1,1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(1,1);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(1,1);
    }
}
