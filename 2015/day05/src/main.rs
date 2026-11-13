use std::{collections::HashMap, fs};

struct Solution {
    part1: i32,
    part2: i32,
}

fn check_nicer(input: &str) -> bool {
    let mut neighbours: bool = false;

    for i in 0..input.len() - 2 {
        let num1 = &input.chars().nth(i);
        let num2 = &input.chars().nth(i + 2);

        if num1 == num2 {
            neighbours = true;
            break;
        }
    }

    let bytes = input.as_bytes();
    let mut seen: HashMap<(u8, u8), usize> = HashMap::new();
    let mut double = false;
    for (i, sub) in bytes.windows(2).enumerate() {
        let pair = (sub[0], sub[1]);
        match seen.get(&pair) {
            Some(&j) if i >= j + 2 => double = true,
            Some(_) => {}
            None => {
                seen.insert(pair, i);
            }
        }
    }

    #[cfg(test)]
    {
        println!("{}", input);
        println!("{}", neighbours);
        println!("{}", double);
    }

    neighbours & double
}

fn check_nice(input: &str) -> bool {
    let mut vowel_count: i32 = 0;
    let mut double_letter: bool = false;
    let mut forbidden_string = false;

    for i in 0..input.len() - 1 {
        let sub = &input[i..i + 2];

        if matches!(sub.chars().next(), Some('a' | 'e' | 'i' | 'o' | 'u')) {
            vowel_count += 1
        }

        let mut double_check = sub.chars();
        if double_check.next() == double_check.next() {
            double_letter = true
        }

        let forbidden = ["ab", "cd", "pq", "xy"];
        if forbidden.contains(&sub) {
            forbidden_string = true;
        }
    }

    if matches!(input.chars().last(), Some('a' | 'e' | 'i' | 'o' | 'u')) {
        vowel_count += 1
    }

    #[cfg(test)]
    {
        println!("{}", vowel_count);
        println!("{}", double_letter);
        println!("{}", forbidden_string);
    }

    vowel_count >= 3 && double_letter && !forbidden_string
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("./src/input.txt")?;
    let input = file.lines();
    let mut solution = Solution { part1: 0, part2: 0 };

    for line in input {
        solution.part1 += if check_nice(line) { 1 } else { 0 };
        solution.part2 += if check_nicer(line) { 1 } else { 0 };
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
        assert_eq!(check_nice("ugknbfddgicrmopn"), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(check_nice("aaa"), true);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(check_nice("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(check_nicer("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(check_nicer("xxyxx"), true);
    }

    #[test]
    fn test_example_6() {
        assert_eq!(check_nicer("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn test_example_7() {
        assert_eq!(check_nicer("ieodomkazucvgmuy"), false);
    }
}
