use std::fs;

fn parse_control_instruction(text: &str, pos: usize) -> Option<(usize, bool)> {
    if pos + 4 > text.len() {
        return None;
    }

    // Check for do() or don't()
    if text[pos..].starts_with("do()") {
        Some((pos + 4, true))
    } else if text[pos..].starts_with("don't()") {
        Some((pos + 7, false))
    } else {
        None
    }
}

fn parse_mul_instruction(text: &str, pos: usize) -> Option<(usize, u32)> {
    // Check if we have enough characters left for a minimal mul(X,Y) instruction
    if pos + 7 > text.len() {
        return None;
    }

    // Check for exact "mul(" prefix
    if !text[pos..].starts_with("mul(") {
        return None;
    }

    let instruction = &text[pos + 4..];
    let mut chars = instruction.chars();

    // Parse first number
    let mut num1 = String::new();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            num1.push(c);
        } else if c == ',' && !num1.is_empty() {
            break;
        } else {
            return None;
        }
    }

    // Parse second number
    let mut num2 = String::new();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            num2.push(c);
        } else if c == ')' && !num2.is_empty() {
            // Valid multiplication found
            if let (Ok(n1), Ok(n2)) = (num1.parse::<u32>(), num2.parse::<u32>()) {
                if (1..=999).contains(&n1) && (1..=999).contains(&n2) {
                    return Some((pos + 4 + num1.len() + 1 + num2.len() + 1, n1 * n2));
                }
            }
            break;
        } else {
            return None;
        }
    }

    None
}

fn process_memory(input: &str) -> u32 {
    let mut pos = 0;
    let mut sum = 0;
    let mut multiply_enabled = true; // Multiplications are enabled by default

    while pos < input.len() {
        // First check for control instructions
        if let Some((new_pos, enabled)) = parse_control_instruction(input, pos) {
            multiply_enabled = enabled;
            pos = new_pos;
            continue;
        }

        // Then check for multiplication instructions
        if let Some((new_pos, result)) = parse_mul_instruction(input, pos) {
            if multiply_enabled {
                sum += result;
            }
            pos = new_pos;
        } else {
            pos += 1;
        }
    }

    sum
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/input3.txt")?;
    let result = process_memory(&input);
    println!("Sum of all enabled multiplication results: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process_memory(input), 161);
    }

    #[test]
    fn test_part2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(process_memory(input), 48);
    }

    #[test]
    fn test_multiple_controls() {
        let input = "mul(2,3)don't()mul(4,5)do()mul(6,7)don't()mul(8,9)";
        assert_eq!(process_memory(input), 48); // 2*3 + 6*7
    }

    #[test]
    fn test_controls_with_invalid_instructions() {
        let input = "do()mul(1,2)don't()mul(3,4]do()mul(5,6)";
        assert_eq!(process_memory(input), 32); // 1*2 + 5*6
    }
}
