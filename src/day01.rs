pub fn day01(input_lines: &str) -> (String, String) {
    let mut mult_res: i32 = 0;

    let lines: Vec<&str> = input_lines.lines().collect();
    let lines_1: Vec<&str> = lines.clone();

    // labelled outter loop to go through inputs
    'outer_loop: for (i, line) in lines_1.iter().enumerate() {
        let line: i32 = line.parse().expect("Not a valid int");

        // inner loop to avoid double counting
        for &later_line in &lines_1[i + 1..] {
            // WHY DOES THIS WORK WITH 1, I.E SAME AS LINE_POS + 1
            let later_line: i32 = later_line.parse().expect("Not valid int");
            //println!("line: {line} later_line {later_line}");

            if (line + later_line) == 2020 {
                mult_res = line * later_line;
                break 'outer_loop;
            }
        }
    }

    let answer1 = mult_res;

    // labelled outter loop to go through inputs
    'outer_loop: for (i, &line) in lines.iter().enumerate() {
        let line: i32 = line.parse().expect("Not a valid int");

        for (p, &later_line) in lines[i + 1..].iter().enumerate() {
            let later_line: i32 = later_line.parse().expect("Not valid int");

            for &laterest_line in lines[p + 1..].iter() {
                let laterest_line: i32 = laterest_line.parse().expect("Not a valid int");
                if (line + later_line + laterest_line) == 2020 {
                    mult_res = line * later_line * laterest_line;
                    break 'outer_loop;
                }
            }
        }
    }

    let answer2 = mult_res;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(
            day01(
                "979
366
1721
299
675
1456"
            )
            .0,
            "514579".to_string()
        )
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(
            day01(
                "1721
979
366
299
675
1456"
            )
            .1,
            "241861950".to_string()
        )
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(
            day01(
                "1721
979
366
299
675
1456"
            )
            .0,
            "514579".to_string()
        );
        assert_eq!(
            day01(
                "1721
979
366
299
675
1456"
            )
            .1,
            "241861950".to_string()
        )
    }
}
