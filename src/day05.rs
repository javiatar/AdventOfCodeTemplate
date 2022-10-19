use std::str::Lines;

fn get_seat_id(row_partition: &str, col_partition: &str) -> usize {
    let row = get_ind_from_bin_part(0, 127, convert_partition_into_instructions(row_partition));
    let col = get_ind_from_bin_part(0, 7, convert_partition_into_instructions(col_partition));

    row * 8 + col
}

fn get_ind_from_bin_part(
    mut lower_bound: usize,
    mut upper_bound: usize,
    part_instructions: Vec<bool>,
) -> usize {
    // mutable bounds are copied (as have copy trait) and can be changed here
    for get_upper_part in &part_instructions {
        let midpoint = ((upper_bound - lower_bound) + 1) / 2;

        // boolean instructions deciding whether to look at top (true) or bottom (false) half of range
        if *get_upper_part {
            lower_bound += midpoint;
        } else {
            upper_bound -= midpoint;
        }
    }

    // need to have converged to 1 exact index
    if lower_bound != upper_bound {
        println!("Instructions provided: {part_instructions:?}");
        panic!("Couldn't converge to one index from instructions' length and bounds provided.")
    } else {
        lower_bound
    }
}

fn convert_partition_into_instructions(partition: &str) -> Vec<bool> {
    // Convert from arbitrary char partition into binary instructins
    // True for top half and false for bottom half recursively
    let mut instructions: Vec<bool> = Vec::new();

    for letter in partition.chars() {
        if ['B', 'R'].contains(&letter) {
            instructions.push(true);
        } else {
            instructions.push(false);
        }
    }
    instructions
}

pub fn day05(input_lines: &str) -> (String, String) {
    let lines_iterator: Lines = input_lines.lines();
    let mut highest_seat_id = 0;
    let mut all_found_seat_ids: Vec<usize> = Vec::new();

    for boarding_pass in lines_iterator {
        let highest_id_candidate = get_seat_id(&boarding_pass[..7], &boarding_pass[7..]);
        if highest_id_candidate > highest_seat_id {
            highest_seat_id = highest_id_candidate;
        }
        all_found_seat_ids.push(highest_id_candidate)
    }
    let answer1 = highest_seat_id;
    let mut answer2 = 0;

    // For part 2 need to collect all ids, sort them, and find the gap in the middle
    all_found_seat_ids.sort();

    for candidate_seat in all_found_seat_ids[0]..=all_found_seat_ids[all_found_seat_ids.len() - 1] {
        // Whichever seat is missing within range of seen seat ids is our seat
        if !all_found_seat_ids.contains(&candidate_seat) {
            answer2 = candidate_seat;
            break;
        }
    }

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(
            day05(
                "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            )
            .0,
            "820".to_string()
        )
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(day05("").1, "0".to_string())
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(day05(""), ("0".to_string(), "0".to_string()))
    }
}
