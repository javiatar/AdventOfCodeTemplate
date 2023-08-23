fn execute_instruction(
    operation: &str,
    argument: &str,
    mut instruction_index: i32,
    mut global_accumulator: i32,
) -> (i32, i32) {
    let argument = argument.parse::<i32>().unwrap();

    if operation == "acc" {
        global_accumulator = add(global_accumulator, argument);
        instruction_index += 1;
    } else if operation == "jmp" {
        instruction_index = add(instruction_index, argument);
    } else if operation == "nop" {
        instruction_index += 1;
    } else {
        panic!()
    }

    (instruction_index, global_accumulator)
}

fn add(u: i32, i: i32) -> i32 {
    if i.is_negative() {
        u - i.wrapping_abs() as i32
    } else {
        u + i as i32
    }
}

pub fn day08_p1(input_lines: &str) -> String {
    let mut instruction_index: i32 = 0;
    let mut global_accumulator: i32 = 0;
    let corrupted_instruction_vec: Vec<&str> = input_lines.lines().collect();
    let mut seen_instructions: Vec<i32> = Vec::new();

    loop {
        // Check if instruction already traversed. If so, we're in an infinite loop.
        if seen_instructions.contains(&instruction_index) {
            println!("Infinite loop detected!");
            break;
        } else {
            seen_instructions.push(instruction_index);
        }

        let instruction_index_usize: usize = instruction_index.try_into().unwrap();
        match corrupted_instruction_vec[instruction_index_usize]
            .split(' ')
            .collect::<Vec<_>>()[..]
        {
            [operation, argument] => {
                (instruction_index, global_accumulator) =
                    execute_instruction(operation, argument, instruction_index, global_accumulator);
            }
            _ => {
                println!("{:?}", corrupted_instruction_vec[instruction_index_usize]);
                panic!()
            }
        }
    }
    format!("{}", global_accumulator)
}

pub fn day08_p2(input_lines: &str) -> String {
    let mut global_accumulator: i32 = 0;
    let corrupted_instruction_vec: Vec<String> =
        input_lines.lines().map(|s| s.to_string()).collect();
    // Will build a vector of all possible instruction vectors. Copies of corrupted original with one jmp<->nop instruction reversed.
    let mut possible_instruction_vecs: Vec<Vec<String>> = Vec::new();
    for (idx, instruction) in corrupted_instruction_vec.iter().enumerate() {
        match &instruction.split(' ').collect::<Vec<_>>()[..] {
            &[operation, argument] => {
                if operation == "jmp" {
                    let new_instruction = format!("nop {}", argument);
                    let mut possible_instruction_vec = corrupted_instruction_vec.clone();
                    possible_instruction_vec[idx] = new_instruction;
                    possible_instruction_vecs.push(possible_instruction_vec);
                } else if operation == "nop" {
                    let new_instruction = format!("jmp {}", argument);
                    let mut possible_instruction_vec = corrupted_instruction_vec.clone();
                    possible_instruction_vec[idx] = new_instruction;
                    possible_instruction_vecs.push(possible_instruction_vec);
                }
            }
            _ => {
                panic!();
            }
        }
    }
    for fixed_instructions_candidate in possible_instruction_vecs {
        let mut seen_instructions: Vec<i32> = Vec::new();
        let mut found_correct_instructions = false;
        let mut instruction_index: i32 = 0;
        global_accumulator = 0;
        loop {
            // Check if instruction already traversed. If so, we're in an infinite loop.
            if seen_instructions.contains(&instruction_index) {
                println!("Infinite loop detected!");
                break;
            } else {
                seen_instructions.push(instruction_index);
            }
            let instruction_index_usize: usize = instruction_index.try_into().unwrap();
            match fixed_instructions_candidate[instruction_index_usize]
                .split(' ')
                .collect::<Vec<_>>()[..]
            {
                [operation, argument] => {
                    println!("instruction_index: {}", instruction_index);
                    println!("global_accumulator: {}", global_accumulator);
                    println!("operation: {}, argument: {}", operation, argument);
                    (instruction_index, global_accumulator) = execute_instruction(
                        operation,
                        argument,
                        instruction_index,
                        global_accumulator,
                    );
                }
                _ => {
                    panic!();
                }
            }
            if instruction_index == fixed_instructions_candidate.len().try_into().unwrap() {
                println!("Found fixed instructions!");
                found_correct_instructions = true;
                break;
            } else if instruction_index > fixed_instructions_candidate.len().try_into().unwrap() {
                panic!("instruction_index > fixed_instructions_candidate.len()")
            } else if instruction_index < 0 {
                panic!("instruction_index < 0")
            }
        }

        if found_correct_instructions {
            break;
        }
    }
    format!("{}", global_accumulator)
}

pub fn day08(input_lines: &str) -> (String, String) {
    let answer1 = day08_p1(input_lines);
    let answer2 = day08_p2(input_lines);
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_part1_case1() {
        assert_eq!(
            day08_p1(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            "5".to_string()
        )
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(
            day08_p2(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            "8".to_string()
        )
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(
            (
                day08_p1(
                    "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
                ),
                day08_p2(
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
            ),
            ("5".to_string(), "8".to_string())
        )
    }
}
