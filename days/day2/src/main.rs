use std::fs;

pub fn solve(input: &str) -> usize {
    input.lines().filter(|line| is_safe_report(line)).count()
}

fn is_safe_report(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    if numbers.len() < 2 {
        return false;
    }

    // Check if sequence is strictly increasing or decreasing
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..numbers.len() - 1 {
        let diff = numbers[i + 1] - numbers[i];

        // Check if difference is between 1 and 3 (inclusive)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff > 0 {
            decreasing = false;
        } else {
            increasing = false;
        }

        // If neither increasing nor decreasing, it's not safe
        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

fn main() {
    let input = fs::read_to_string("input/input2.txt").expect("Error reading input file");
    let result = solve(&input);
    println!("Number of safe reports: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_individual_cases() {
        assert!(is_safe_report("7 6 4 2 1")); // Decreasing by 1 or 2
        assert!(!is_safe_report("1 2 7 8 9")); // Increase of 5
        assert!(!is_safe_report("9 7 6 2 1")); // Decrease of 4
        assert!(!is_safe_report("1 3 2 4 5")); // Mixed increase/decrease
        assert!(!is_safe_report("8 6 4 4 1")); // No change between numbers
        assert!(is_safe_report("1 3 6 7 9")); // Increasing by 1, 2, or 3
    }
}
