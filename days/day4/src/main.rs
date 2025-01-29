use std::fs;

fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // Define all possible directions to search
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (-1, 1),  // diagonal up-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
    ];

    // Check each starting position
    for y in 0..height {
        for x in 0..width {
            // Try each direction from this position
            for &(dy, dx) in &directions {
                let mut found = true;
                let target = ['X', 'M', 'A', 'S'];

                // Check if we can fit XMAS in this direction without going out of bounds
                for i in 0..4 {
                    let new_y = y as i32 + dy * i;
                    let new_x = x as i32 + dx * i;

                    if new_y < 0 || new_y >= height as i32 || new_x < 0 || new_x >= width as i32 {
                        found = false;
                        break;
                    }

                    if grid[new_y as usize][new_x as usize] != target[i as usize] {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    let input = fs::read_to_string("input/input4.txt")?;

    // Parse the grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Count occurrences of XMAS
    let result = count_xmas(&grid);

    println!("XMAS appears {} times in the word search", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![
            "MMMSXXMASM".chars().collect(),
            "MSAMXMSMSA".chars().collect(),
            "AMXSXMAAMM".chars().collect(),
            "MSAMASMSMX".chars().collect(),
            "XMASAMXAMM".chars().collect(),
            "XXAMMXXAMA".chars().collect(),
            "SMSMSASXSS".chars().collect(),
            "SAXAMASAAA".chars().collect(),
            "MAMMMXMMMM".chars().collect(),
            "MXMXAXMASX".chars().collect(),
        ];

        assert_eq!(count_xmas(&input), 18);
    }
}
