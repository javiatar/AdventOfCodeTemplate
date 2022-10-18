use std::str::Lines;

fn trees_in_slope(right_offset: usize, down_offset: usize, lines_iterator: Lines<'_>) -> usize {
    let mut cyclic_position = right_offset;
    let mut trees_in_los = 0;
    let mut next_line_offset = 0;

    for line in lines_iterator {
        if next_line_offset == down_offset {
            // Cyclical mountain => use modulus operator to wrap-around each line
            cyclic_position %= line.len();

            if line.chars().nth(cyclic_position).unwrap() == '#' {
                trees_in_los += 1;
            };

            cyclic_position += right_offset;
            next_line_offset = 0;
        }

        next_line_offset += 1;
    }
    trees_in_los
}

pub fn day03(input_lines: &str) -> (String, String) {
    let lines_iterator: Lines = input_lines.lines();

    let answer1 = trees_in_slope(3, 1, lines_iterator.clone());

    let trees_r1_d1 = trees_in_slope(1, 1, lines_iterator.clone());
    let trees_r3_d1 = answer1;
    let trees_r5_d1 = trees_in_slope(5, 1, lines_iterator.clone());
    let trees_r7_d1 = trees_in_slope(7, 1, lines_iterator.clone());
    let trees_r1_d2 = trees_in_slope(1, 2, lines_iterator);

    let answer2 = trees_r1_d1 * trees_r3_d1 * trees_r5_d1 * trees_r7_d1 * trees_r1_d2;

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(
            day03(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            )
            .0,
            "7".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            day03(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            )
            .1,
            "336".to_string()
        )
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(
            day03(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            ),
            ("7".to_string(), "336".to_string())
        )
    }
}
