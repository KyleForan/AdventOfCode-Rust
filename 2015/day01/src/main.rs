use std::error::Error;
use std::fs;

struct Solution {
    part1: i32,
    part2: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let results = fs::read_to_string("./src/input.txt")?;
    let mut solution = Solution { part1: 0, part2: 0 };

    for (ind, chr) in results.chars().enumerate() {
        match chr {
            '(' => solution.part1 += 1,
            ')' => solution.part1 -= 1,
            _ => panic!(""),
        }

        if solution.part1 < 0 && solution.part2 == 0 {
            solution.part2 = (ind + 1) as i32
        }
    }

    println!("Part 1 Solution: {}", solution.part1);
    println!("Part 1 Solution: {}", solution.part2);

    Ok(())
}
