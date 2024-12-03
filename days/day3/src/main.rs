use std::fs;

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

    while pos < input.len() {
        if let Some((new_pos, result)) = parse_mul_instruction(input, pos) {
            sum += result;
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
    println!("Sum of all multiplication results: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process_memory(input), 161);
    }

    #[test]
    fn test_invalid_instructions() {
        let input = "mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 )";
        assert_eq!(process_memory(input), 0);
    }

    #[test]
    fn test_valid_range() {
        let input = "mul(1,1)mul(999,999)mul(0,5)mul(1000,5)";
        assert_eq!(process_memory(input), 999000); // Only 1*1 + 999*999
    }
}
