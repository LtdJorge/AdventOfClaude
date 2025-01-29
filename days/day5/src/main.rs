use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input5.txt").expect("Failed to read input file");
    let result = solve_part1(&input);
    println!("Sum of middle page numbers: {}", result);
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn middle_page(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }
}

fn parse_input(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Update>) {
    let mut parts = input.split("\n\n");

    // Parse rules
    let rules_str = parts.next().unwrap();
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    for line in rules_str.lines() {
        if line.is_empty() {
            continue;
        }
        let mut nums = line.split('|');
        let before: u32 = nums.next().unwrap().parse().unwrap();
        let after: u32 = nums.next().unwrap().parse().unwrap();

        rules.entry(before).or_default().insert(after);
    }

    // Parse updates
    let updates_str = parts.next().unwrap();
    let updates = updates_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Update {
            pages: line.split(',').map(|n| n.parse().unwrap()).collect(),
        })
        .collect();

    (rules, updates)
}

fn is_valid_order(pages: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for (i, &page) in pages.iter().enumerate() {
        if let Some(must_follow) = rules.get(&page) {
            // Check that all pages that must follow this one actually do
            let remaining_pages: HashSet<_> = pages[i + 1..].iter().copied().collect();
            for &after_page in must_follow {
                if pages.contains(&after_page) && !remaining_pages.contains(&after_page) {
                    return false;
                }
            }
        }

        // Check that this page doesn't violate any other page's rules
        for (&before_page, after_pages) in rules {
            if after_pages.contains(&page) && pages.contains(&before_page) {
                let before_idx = pages.iter().position(|&x| x == before_page).unwrap();
                if before_idx > i {
                    return false;
                }
            }
        }
    }
    true
}

fn solve_part1(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| is_valid_order(&update.pages, &rules))
        .map(|update| update.middle_page())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(solve_part1(input), 143);
    }
}
