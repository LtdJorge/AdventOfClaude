use std::fs;

fn count_x_mas(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // Check each position as the center of potential X
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[y][x] != 'A' {
                continue;
            }

            // For each diagonal direction pair that forms an X
            let diagonals = [
                // top-left to bottom-right paired with top-right to bottom-left
                (((-1, -1), (1, 1)), ((-1, 1), (1, -1))),
            ];

            for &(dir1, dir2) in &diagonals {
                count += check_x_pattern(grid, y, x, dir1, dir2);
            }
        }
    }

    count
}

fn check_x_pattern(
    grid: &Vec<Vec<char>>,
    center_y: usize,
    center_x: usize,
    dir1: ((i32, i32), (i32, i32)),
    dir2: ((i32, i32), (i32, i32)),
) -> usize {
    let mut patterns = 0;

    // Check both possible orientations for first diagonal
    let mas1_forward = check_mas(grid, center_y, center_x, dir1.0, dir1.1);
    let mas1_backward = check_mas(grid, center_y, center_x, dir1.1, dir1.0);

    // Check both possible orientations for second diagonal
    let mas2_forward = check_mas(grid, center_y, center_x, dir2.0, dir2.1);
    let mas2_backward = check_mas(grid, center_y, center_x, dir2.1, dir2.0);

    // Count valid combinations
    if mas1_forward && mas2_forward {
        patterns += 1;
    }
    if mas1_forward && mas2_backward {
        patterns += 1;
    }
    if mas1_backward && mas2_forward {
        patterns += 1;
    }
    if mas1_backward && mas2_backward {
        patterns += 1;
    }

    patterns
}

fn check_mas(
    grid: &Vec<Vec<char>>,
    center_y: usize,
    center_x: usize,
    start_dir: (i32, i32),
    end_dir: (i32, i32),
) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    // Check M position
    let m_y = center_y as i32 + start_dir.0;
    let m_x = center_x as i32 + start_dir.1;
    if m_y < 0
        || m_y >= height
        || m_x < 0
        || m_x >= width
        || grid[m_y as usize][m_x as usize] != 'M'
    {
        return false;
    }

    // Check S position
    let s_y = center_y as i32 + end_dir.0;
    let s_x = center_x as i32 + end_dir.1;
    if s_y < 0
        || s_y >= height
        || s_x < 0
        || s_x >= width
        || grid[s_y as usize][s_x as usize] != 'S'
    {
        return false;
    }

    // If we got here, we found a valid MAS sequence
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    let input = fs::read_to_string("input/input4.txt")?;

    // Parse the grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Count occurrences of X-MAS
    let result = count_x_mas(&grid);

    println!("X-MAS appears {} times in the word search", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![
            ".M.S......".chars().collect(),
            "..A..MSMS.".chars().collect(),
            ".M.S.MAA..".chars().collect(),
            "..A.ASMSM.".chars().collect(),
            ".M.S.M....".chars().collect(),
            "..........".chars().collect(),
            "S.S.S.S.S.".chars().collect(),
            ".A.A.A.A..".chars().collect(),
            "M.M.M.M.M.".chars().collect(),
            "..........".chars().collect(),
        ];

        assert_eq!(count_x_mas(&input), 9);
    }

    #[test]
    fn test_simple_x() {
        let input = vec![
            "M.S".chars().collect(),
            ".A.".chars().collect(),
            "M.S".chars().collect(),
        ];

        assert_eq!(count_x_mas(&input), 1);
    }
}
