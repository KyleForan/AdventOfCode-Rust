use grid::Grid;
use std::fs;

struct Solution {
    part1: i32,
    part2: i32,
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Default, Debug)]
struct Light {
    status: i32,
}

#[derive(Debug)]
struct LightGrid {
    grid: Grid<Light>,
}

impl LightGrid {
    fn new() -> LightGrid {
        Self {
            grid: Grid::new(1000, 1000),
        }
    }

    fn adjust_brightness(&mut self, input: &str) {
        let (action, (sx, sy), (ex, ey)) = match self.parse_input(&input) {
            Some(results) => results,
            _ => return,
        };

        #[cfg(test)]
        println!("{:?} {},{} {},{}", action, sx, sy, ex, ey);

        for y in sx..=ex {
            for x in sy..=ey {
                let bulb = &mut self.grid[(y, x)];
                match action {
                    Action::TurnOn => bulb.status += 1,
                    Action::TurnOff => bulb.status -= if bulb.status > 0 { 1 } else { 0 },
                    Action::Toggle => bulb.status += 2,
                }
            }
        }
    }

    fn toggle_light(&mut self, input: &str) {
        let (action, (sx, sy), (ex, ey)) = match self.parse_input(&input) {
            Some(results) => results,
            _ => return,
        };

        #[cfg(test)]
        println!("{:?} {},{} {},{}", action, sx, sy, ex, ey);

        for y in sx..=ex {
            for x in sy..=ey {
                let bulb = &mut self.grid[(y, x)];
                match action {
                    Action::TurnOn => bulb.status = 1,
                    Action::TurnOff => bulb.status = 0,
                    Action::Toggle => bulb.status ^= 1,
                }
            }
        }
    }

    fn parse_input(&self, input: &str) -> Option<(Action, (usize, usize), (usize, usize))> {
        // turn on 0,0 through 999,999
        let (action, rest) = if let Some(r) = input.strip_prefix("turn on ") {
            (Action::TurnOn, r)
        } else if let Some(r) = input.strip_prefix("turn off ") {
            (Action::TurnOff, r)
        } else if let Some(r) = input.strip_prefix("toggle ") {
            (Action::Toggle, r)
        } else {
            return None;
        };

        #[cfg(test)]
        println!("{:?} {}", action, rest);

        let nums: Vec<usize> = rest
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|c| c.parse().ok())
            .collect();

        #[cfg(test)]
        println!("{:?}", nums);

        let [a, b, c, d] = nums[..] else { return None };

        Some((action, (a, b), (c, d)))
    }

    fn count_lights(&self) -> usize {
        self.grid.iter().map(|x| x.status).sum::<i32>() as usize
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("./src/input.txt")?;
    let input = file.lines();
    let mut solution = Solution { part1: 0, part2: 0 };

    let mut light_grid: LightGrid = LightGrid::new();
    let mut bright_grid: LightGrid = LightGrid::new();

    for line in input {
        light_grid.toggle_light(&line);
        bright_grid.adjust_brightness(&line);
    }

    solution.part1 = light_grid.count_lights() as i32;
    solution.part2 = bright_grid.count_lights() as i32;

    println!("{}", solution.part1);
    println!("{}", solution.part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut lightgrid = LightGrid::new();
        lightgrid.toggle_light("turn on 0,0 through 999,999");

        assert_eq!(lightgrid.count_lights(), 1000 * 1000);
    }

    #[test]
    fn test_example_2() {
        let mut lightgrid = LightGrid::new();
        lightgrid.toggle_light("toggle 0,0 through 999,0");

        assert_eq!(lightgrid.count_lights(), 1000);
    }

    #[test]
    fn test_example_3() {
        let mut lightgrid = LightGrid::new();
        lightgrid.toggle_light("turn on 0,0 through 999,999");
        lightgrid.toggle_light("turn off 499,499 through 500,500");

        assert_eq!(lightgrid.count_lights(), 1000000 - 4);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(1, 1);
    }
}
