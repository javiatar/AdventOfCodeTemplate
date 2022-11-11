use itertools::Itertools;
use std::collections::HashSet;

fn get_num_unique_qs_in_group(ppl_answers: &str) -> usize {
    // Want num uniquely appearing letters in entire group (ignore newlines b/w ppl)
    ppl_answers.chars().filter(|&c| c != '\n').unique().count()
}

fn get_num_common_qs(ppl_answers: &str) -> usize {
    let mut sets: Vec<HashSet<char>> = Vec::new();

    // Convert group answers to vector of HashSets of each person's answers
    ppl_answers.split('\n').for_each(|ans| {
        sets.push(ans.chars().collect::<HashSet<char>>());
    });

    let mut iter = sets.into_iter();

    let first = iter.next();

    let intersection = first.map(|set| iter.fold(set, |set1, set2| &set1 & &set2));

    intersection.unwrap_or_default().len()
}

pub fn day06(input_lines: &str) -> (String, String) {
    // Sum unique qs for each group (split by \n\n)
    let answer1: usize = input_lines
        .split("\n\n")
        .map(get_num_unique_qs_in_group)
        .sum();

    let answer2: usize = input_lines.split("\n\n").map(get_num_common_qs).sum();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(
            day06(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )
            .0,
            "11".to_string()
        )
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(
            day06(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )
            .1,
            "6".to_string()
        )
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(day06(""), ("0".to_string(), "0".to_string()))
    }
}
