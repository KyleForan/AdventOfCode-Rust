use std::{collections::HashSet, error::Error, fs, hash::Hash};

struct Solution {
    part1: i32,
    part2: i32,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Santa {
    house: Point,
    seen: HashSet<Point>,
}

impl Santa {
    fn new() -> Self {
        let mut seen: HashSet<Point> = HashSet::new();
        let house = Point::new(0, 0);
        seen.insert(house);

        Self { house, seen }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[cfg(test)]
    fn get_cords(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn next_house(&mut self, chr: char) {
        let (x, y) = match chr {
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, -1),
            '^' => (0, 1),
            _ => (0, 0),
        };

        self.x += x;
        self.y += y;
    }
}

fn get_houses(instuctions: String) -> i32 {
    // let mut position = Point::new(0,0);
    // let mut seen: HashSet<Point> = HashSet::new();

    let mut santa = Santa::new();

    for chr in instuctions.chars() {
        santa.house.next_house(chr);
        let point = santa.house;
        santa.seen.insert(point);

        #[cfg(test)]
        {
            let (x, y) = santa.house.get_cords();
            println!("{} {}", x, y);
            println!("{:?}", santa);
        }
    }

    santa.seen.len() as i32
}

fn get_houses_with_robot(instuctions: String) -> i32 {
    // let mut position = Point::new(0,0);
    // let mut seen: HashSet<Point> = HashSet::new();

    let mut santa = Santa::new();
    let mut robot = Santa::new();

    for (i, chr) in instuctions.chars().enumerate() {
        let current_santa = if i % 2 == 0 { &mut santa } else { &mut robot };

        #[cfg(test)]
        println!("Current Santa {}", i % 2);

        current_santa.house.next_house(chr);
        let point = current_santa.house;
        current_santa.seen.insert(point);
    }

    #[cfg(test)]
    {
        println!("Robot: {:?}", robot);
        println!("Santa: {:?}", santa);
    }

    santa.seen.union(&robot.seen).count() as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut solution = Solution { part1: 0, part2: 0 };

    let file = fs::read_to_string("./src/input.txt")?;

    solution.part1 = get_houses(file.clone());
    solution.part2 = get_houses_with_robot(file.clone());

    println!("{}", solution.part1);
    println!("{}", solution.part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_houses, get_houses_with_robot};

    #[test]
    fn example_1() {
        let answer = get_houses(">".to_string());
        assert_eq!(answer, 2)
    }

    #[test]
    fn example_2() {
        let answer = get_houses("^>v<".to_string());
        assert_eq!(answer, 4)
    }

    #[test]
    fn example_3() {
        let answer = get_houses("^v^v^v^v^v".to_string());
        assert_eq!(answer, 2)
    }

    #[test]
    fn example_4() {
        let answer = get_houses_with_robot("^v".to_string());
        assert_eq!(answer, 3)
    }

    #[test]
    fn example_5() {
        let answer = get_houses_with_robot("^>v<".to_string());
        assert_eq!(answer, 3)
    }

    #[test]
    fn example_6() {
        let answer = get_houses_with_robot("^v^v^v^v^v".to_string());
        assert_eq!(answer, 11)
    }
}
