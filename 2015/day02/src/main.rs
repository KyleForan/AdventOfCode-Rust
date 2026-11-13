use std::fs;

struct Solution {
    part1: i32,
    part2: i32,
}

struct Prism {
    h: i32,
    w: i32,
    l: i32,
}

impl Prism {
    fn new(dimensions: &str) -> Self {
        let dims: Vec<i32> = dimensions
            .split('x')
            .filter_map(|x| x.parse().ok())
            .collect();
        let [h, w, l] = dims[..] else {
            panic!("Wrong Format: {:?}", dims);
        };

        Self { h, w, l }
    }

    fn area(&self) -> i32 {
        let sides = [
            (2 * self.l * self.w),
            (2 * self.w * self.h),
            (2 * self.l * self.h),
        ];

        let smallest_side: i32 = *sides.iter().min().unwrap() / 2;
        let area: i32 = sides.iter().sum();

        area + smallest_side
    }

    fn perimiter(&self) -> i32 {
        let mut items = [self.h, self.w, self.l];
        items.sort();

        let [num1, num2, num3] = items;

        #[cfg(test)]
        println!("items: {} {} {}", num1, num2, num3);

        (num1 * 2) + (num2 * 2) + (num1 * num2 * num3)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("./src/input.txt")?;
    let input = file.lines();
    let mut solution = Solution { part1: 0, part2: 0 };

    for line in input {
        let prism = Prism::new(line);

        solution.part1 += prism.area();
        solution.part2 += prism.perimiter();
    }

    println!("{}", solution.part1);
    println!("{}", solution.part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_1() {
        let prism = Prism::new("2x3x4");
        assert_eq!(prism.area(), 58)
    }

    #[test]
    fn test_area_2() {
        let prism = Prism::new("1x1x10");
        assert_eq!(prism.area(), 43)
    }

    #[test]
    fn test_perimiter_1() {
        let prism = Prism::new("2x3x4");
        assert_eq!(prism.perimiter(), 34)
    }

    #[test]
    fn test_perimiter_2() {
        let prism = Prism::new("1x1x10");
        assert_eq!(prism.perimiter(), 14)
    }
}
