use std::{cmp::Ordering, io};

pub fn day09(input_lines: &str) -> (String, String) {
    let mut answer1: usize = 0;
    let mut preamble_size = String::new();
    let input_numbers: Vec<usize> = input_lines
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut preamble: Vec<usize> = Vec::new();

    println!("Enter the preamble size:");
    io::stdin()
        .read_line(&mut preamble_size)
        .expect("Failed to read preamble size");
    let preamble_size: usize = preamble_size.trim().parse().unwrap_or(25);
    println!("Preamble size: {}", preamble_size);

    // copy preamble_size numbers from input into preamble vec
    preamble.extend_from_slice(&input_numbers[0..preamble_size]);

    // add all additions of preamble numbers to preamble_addition_combos
    let mut preamble_addition_combos: Vec<usize> = Vec::new();

    // loop over input numbers not in preamble.
    for &number in input_numbers[preamble_size..].iter() {
        // build combinations of additions of existing preamble numbers.
        for i in 0..preamble_size {
            for j in i + 1..preamble_size {
                preamble_addition_combos.push(preamble.get(i).unwrap() + preamble.get(j).unwrap());
            }
        }
        println!("Input number: {}", number);
        println!("preamble: {:?}", preamble);
        println!("preamble_addition_combos: {:?}", preamble_addition_combos);

        // check if number is missing from preamble_addition_combos
        if !preamble_addition_combos.contains(&number) {
            answer1 = number;
            println!("\n CONGRATS - Found answer1: {}! \n", answer1);
            break;
        }

        // otherwise discard first number from preamble
        let _ = preamble.remove(0);
        // add valid number to preamble
        preamble.push(number);
        // and clear preamble addition combinations
        preamble_addition_combos.clear();
    }

    // part 2
    let mut contiguous_numbers: Vec<usize> = Vec::new();

    let mut contiguous_numbers_start_idx = 0;
    let mut input_numbers_idx = contiguous_numbers_start_idx;

    // loop over input numbers
    loop {
        let mut number = input_numbers[input_numbers_idx];
        println!("Input number: {}", number);
        // add number to contiguous_numbers
        contiguous_numbers.push(number);
        println!("contiguous_numbers: {:?}", contiguous_numbers);

        let contiguous_numbers_sum = contiguous_numbers.iter().sum::<usize>();
        println!("contiguous_numbers_sum: {}", contiguous_numbers_sum);

        match contiguous_numbers_sum.cmp(&answer1) {
            // if contiguous_numbers sum to answer1, return sum of min and max
            Ordering::Equal => {
                let answer2 = contiguous_numbers.iter().min().unwrap()
                    + contiguous_numbers.iter().max().unwrap();
                println!("contiguous_numbers: {:?}\n", contiguous_numbers);
                return (format!("{}", answer1), format!("{}", answer2));
            }

            // if contiguous_numbers sum to more than answer1, start contiguous_numbers
            // attempt from next number
            Ordering::Greater => {
                println!("Oops, too big!");
                contiguous_numbers.clear();
                contiguous_numbers_start_idx += 1;
                input_numbers_idx = contiguous_numbers_start_idx;
            }
            Ordering::Less => {
                println!("Oops, too small!");
                input_numbers_idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(
            day09(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
            )
            .0,
            "127".to_string()
        )
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(
            day09(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
            )
            .1,
            "62".to_string()
        )
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(
            day09(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
            ),
            ("127".to_string(), "62".to_string())
        )
    }
}
