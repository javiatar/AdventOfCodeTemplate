use std::{collections::HashSet, str::Lines};

fn get_seat_id(row_partition: &str, col_partition: &str) -> usize {
    let row = get_ind_from_bin_part(0, 127, &convert_partition_into_instructions(row_partition));
    let col = get_ind_from_bin_part(0, 7, &convert_partition_into_instructions(col_partition));

    row * 8 + col
}

fn get_ind_from_bin_part(
    mut lower_bound: usize,
    mut upper_bound: usize,
    part_instructions: &Vec<bool>,
) -> usize {
    // mutable bounds are copied (as have copy trait) and can be changed here
    for &get_upper_part in part_instructions {
        let midpoint = ((upper_bound - lower_bound) + 1) / 2;

        // boolean instructions deciding whether to look at top (true) or bottom (false) half of range
        if get_upper_part {
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
    // Convert from arbitrary char partition into binary instructions
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

    let all_found_seat_ids: Vec<usize> = lines_iterator
        .map(|boarding_pass| get_seat_id(&boarding_pass[..7], &boarding_pass[7..]))
        .collect();

    let &largest_seen_seat_id = all_found_seat_ids.iter().max().unwrap();
    let &smallest_seen_seat_id = all_found_seat_ids.iter().min().unwrap();

    // For part 2 get gap between possible seats set and actually seen id set
    let all_found_seat_ids: HashSet<usize> = HashSet::from_iter(all_found_seat_ids);
    let all_possible_seat_ids: HashSet<usize> =
        HashSet::from_iter(smallest_seen_seat_id..=largest_seen_seat_id);

    let answer1 = largest_seen_seat_id;
    let answer2 = all_possible_seat_ids
        .difference(&all_found_seat_ids)
        .next()
        .unwrap();
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
}
