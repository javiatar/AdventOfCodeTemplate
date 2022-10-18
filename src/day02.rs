use lazy_static::lazy_static;
use regex::Regex;

use std::str::Lines;

struct PasswdPolicy {
    first_ind: usize,
    sec_ind: usize,
    needed_char: char,
    pwd: String,
}

impl PasswdPolicy {
    fn new(pwd: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)").unwrap();
        }

        let caps = RE.captures(pwd).unwrap();

        Self {
            first_ind: caps[1].parse().expect("Couldn't parse start_pos to int"),
            sec_ind: caps[2].parse().expect("Couldn't parse end_pos to int"),
            needed_char: caps[3].chars().next().unwrap(), // iterator over chars moved to 1st position for 1 char &str
            pwd: String::from(&caps[4]),
        }
    }

    fn is_pwd_valid_task1(&self) -> bool {
        // Need number of times needed_char appears in the attempted pwd
        let no_ocurrences: usize = self.pwd.matches(self.needed_char).count();

        // Task 1 interprets two indices as bounds for num times needed char needs to be found
        (self.first_ind..=self.sec_ind).contains(&no_ocurrences)
    }

    fn is_pwd_valid_task2(&self) -> bool {
        // Task 2 interprets inds as pos, where needed char must only appear in one of them. Uses XOR.
        return (self.pwd.chars().nth(self.first_ind - 1).unwrap() == self.needed_char)
            ^ (self.pwd.chars().nth(self.sec_ind - 1).unwrap() == self.needed_char);
    }
}

pub fn day02(input_lines: &str) -> (String, String) {
    let lines_iterator: Lines = input_lines.lines();

    let mut count_valid_passwd_task1: u32 = 0;
    let mut count_valid_passwd_task2: u32 = 0;

    for pwd in lines_iterator {
        let pwd_pol = PasswdPolicy::new(pwd);

        if pwd_pol.is_pwd_valid_task1() {
            count_valid_passwd_task1 += 1;
        }

        if pwd_pol.is_pwd_valid_task2() {
            count_valid_passwd_task2 += 1
        }
    }

    let answer1: u32 = count_valid_passwd_task1;
    let answer2: u32 = count_valid_passwd_task2;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(
            day02(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            )
            .0,
            "2".to_string()
        )
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(
            day02(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            )
            .1,
            "1".to_string()
        )
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(day02(""), ("0".to_string(), "0".to_string()))
    }
}
