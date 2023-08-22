// fn parse_instruction(instruction: &str) -> (usize, usize) {}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        // GitHub Copilot: The reason why `u - i.wrapping_abs() as u32 as usize` requires two `as` statements is because the `wrapping_abs()` method
        // returns an `i32` value, but we need to convert it to a `u32` value before we can convert it to a `usize` value. usize = u32 only on 32-bit platforms.
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

pub fn day08(input_lines: &str) -> (String, String) {
    let mut instruction_index: usize = 0;
    let mut global_acc: usize = 0;
    let instruction_vec: Vec<&str> = input_lines.lines().collect();
    let mut seen_instructions: Vec<usize> = Vec::new();

    loop {
        match &instruction_vec[instruction_index]
            .split(' ')
            .collect::<Vec<_>>()[..]
        {
            &[operation, argument] => {
                let argument = argument.parse::<i32>().unwrap();

                if seen_instructions.contains(&instruction_index) {
                    break;
                } else {
                    seen_instructions.push(instruction_index);
                }

                if operation == "acc" {
                    global_acc = add(global_acc, argument);
                    instruction_index += 1;
                } else if operation == "jmp" {
                    instruction_index = add(instruction_index, argument);
                } else if operation == "nop" {
                    instruction_index += 1;
                } else {
                    panic!()
                }
            }
            _ => {
                panic!()
            }
        }
    }
    let answer1 = global_acc;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_part1_case1() {
        assert_eq!(
            day08(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            )
            .0,
            "5".to_string()
        )
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(day08("").1, "0".to_string())
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(day08(""), ("0".to_string(), "0".to_string()))
    }
}
