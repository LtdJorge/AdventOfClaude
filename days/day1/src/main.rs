use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct HistorianLists {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl FromStr for HistorianLists {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in s.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let numbers: Vec<&str> = line.split_whitespace().collect();
            if numbers.len() != 2 {
                return Err(
                    "Invalid input format: each line must contain exactly two numbers".into(),
                );
            }

            left.push(numbers[0].parse::<i32>()?);
            right.push(numbers[1].parse::<i32>()?);
        }

        Ok(HistorianLists { left, right })
    }
}

impl HistorianLists {
    fn calculate_total_distance(&self) -> i32 {
        let mut left_sorted = self.left.clone();
        let mut right_sorted = self.right.clone();

        left_sorted.sort_unstable();
        right_sorted.sort_unstable();

        left_sorted
            .iter()
            .zip(right_sorted.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }
}

pub fn solve_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let lists = HistorianLists::from_str(input)?;
    Ok(lists.calculate_total_distance())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let result = solve_part1(&input)?;
    println!("Solution: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        assert_eq!(solve_part1(input).unwrap(), 11);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        assert!(solve_part1(input).is_ok());
    }

    #[test]
    fn test_invalid_input() {
        let input = "3 4 5\n4 3";
        assert!(solve_part1(input).is_err());
    }
}
