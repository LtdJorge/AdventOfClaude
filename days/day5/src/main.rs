use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input5.txt").expect("Failed to read input file");
    let result1 = solve_part1(&input);
    println!(
        "Part 1 - Sum of middle page numbers from valid updates: {}",
        result1
    );

    let result2 = solve_part2(&input);
    println!(
        "Part 2 - Sum of middle page numbers from corrected invalid updates: {}",
        result2
    );
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

fn topological_sort(pages: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    // Build adjacency list and in-degree count for present pages
    let mut adj_list: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    let pages_set: HashSet<_> = pages.iter().copied().collect();

    // Initialize in-degree to 0 for all pages
    for &page in pages {
        in_degree.insert(page, 0);
        adj_list.insert(page, HashSet::new());
    }

    // Build graph from rules, only considering pages present in the update
    for (&from, to_pages) in rules {
        if pages_set.contains(&from) {
            for &to in to_pages {
                if pages_set.contains(&to) {
                    adj_list.entry(from).or_default().insert(to);
                    *in_degree.entry(to).or_default() += 1;
                }
            }
        }
    }

    // Kahn's algorithm for topological sort
    let mut result = Vec::with_capacity(pages.len());
    let mut queue: VecDeque<_> = in_degree
        .iter()
        // this pattern relies on behavior which may change in edition 2024 cannot implicitly match against multiple layers of reference Note: for more information, see <https:// doc. rust-lang. org/ nightly/ edition-guide/ rust-2024/ match-ergonomics. html>
        // .filter(|(_, &count)| count == 0)
        .filter(|&(_, &count)| count == 0)
        .map(|(&page, _)| page)
        .collect();

    while let Some(page) = queue.pop_front() {
        result.push(page);

        if let Some(neighbors) = adj_list.get(&page) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    result
}

fn solve_part1(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| is_valid_order(&update.pages, &rules))
        .map(|update| update.middle_page())
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| !is_valid_order(&update.pages, &rules))
        .map(|update| {
            let ordered = topological_sort(&update.pages, &rules);
            Update { pages: ordered }.middle_page()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
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

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 123);
    }
}
