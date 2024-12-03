use std::collections::HashMap;
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

    fn calculate_similarity_score(&self) -> i32 {
        // Create frequency map for right list
        let right_frequencies: HashMap<i32, i32> =
            self.right.iter().fold(HashMap::new(), |mut map, &num| {
                *map.entry(num).or_insert(0) += 1;
                map
            });

        // Calculate similarity score
        self.left
            .iter()
            .map(|&num| num * right_frequencies.get(&num).unwrap_or(&0))
            .sum()
    }
}

pub fn solve_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let lists = HistorianLists::from_str(input)?;
    Ok(lists.calculate_total_distance())
}

pub fn solve_part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let lists = HistorianLists::from_str(input)?;
    Ok(lists.calculate_similarity_score())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let part1_result = solve_part1(&input)?;
    println!("Part 1 solution: {}", part1_result);

    let part2_result = solve_part2(&input)?;
    println!("Part 2 solution: {}", part2_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE_INPUT).unwrap(), 11);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE_INPUT).unwrap(), 31);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        assert!(solve_part1(input).is_ok());
        assert!(solve_part2(input).is_ok());
    }

    #[test]
    fn test_invalid_input() {
        let input = "3 4 5\n4 3";
        assert!(solve_part1(input).is_err());
        assert!(solve_part2(input).is_err());
    }
}
