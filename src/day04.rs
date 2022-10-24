use std::collections::{HashMap, HashSet};

use once_cell::sync::OnceCell;
use regex::Regex;

struct PassportCandidate<'a> {
    pass_fields: HashMap<&'a str, &'a str>,
    tag_set: HashSet<&'a str>,
}

impl PassportCandidate<'_> {
    fn new(passport_candidate: &str) -> PassportCandidate {
        let pass_fields: HashMap<&str, &str> = passport_candidate
            .split_whitespace()
            .map(PassportCandidate::get_tag_and_field)
            .collect();

        let tag_set = pass_fields.keys().cloned().collect();

        PassportCandidate {
            pass_fields,
            tag_set,
        }
    }

    fn has_all_passport_fields(&self, exp_tags: &HashSet<&str>) -> bool {
        self.tag_set.is_superset(exp_tags)
    }

    fn get_tag_and_field(field: &str) -> (&str, &str) {
        // split field into tag and field contents, skipping the ':' in between
        let split_field = field.split_at(3);
        (split_field.0, &split_field.1[1..])
    }

    fn is_field_exp_len(field: &str, len: usize) -> bool {
        field.len() == len
    }

    fn is_num_in_range_inclusive(num: &str, lb_num: usize, ub_num: usize) -> bool {
        let num: usize = num.parse().expect("is not valid number format");

        (lb_num..=ub_num).contains(&num)
    }
}

impl PassportCandidate<'_> {
    fn is_byr_valid(field_contents: &str) -> bool {
        // check byr is within time-frame: 1920 to 2002
        Self::is_field_exp_len(field_contents, 4)
            && Self::is_num_in_range_inclusive(field_contents, 1920, 2002)
    }

    fn is_iyr_valid(field_contents: &str) -> bool {
        // check iyr is within time-frame: 2010 to 2020
        Self::is_field_exp_len(field_contents, 4)
            && Self::is_num_in_range_inclusive(field_contents, 2010, 2020)
    }

    fn is_eyr_valid(field_contents: &str) -> bool {
        // check eyr is within time-frame: 2020 to 2030
        Self::is_field_exp_len(field_contents, 4)
            && Self::is_num_in_range_inclusive(field_contents, 2020, 2030)
    }

    fn is_hgt_valid(field_contents: &str) -> bool {
        let (measurement, unit): (&str, &str) = field_contents.split_at(field_contents.len() - 2);

        match unit {
            "cm" => Self::is_num_in_range_inclusive(measurement, 150, 193),
            "in" => Self::is_num_in_range_inclusive(measurement, 59, 76),
            _ => false,
        }
    }

    fn is_hcl_valid(field_contents: &str) -> bool {
        static RE: OnceCell<Regex> = OnceCell::new();

        // Match only a # and 6 characters [a-f] and/or [0-9]
        RE.get_or_init(|| Regex::new(r"^#[a-f0-9]{6}$").unwrap())
            .is_match(field_contents)
    }
    fn is_ecl_valid(field_contents: &str) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&field_contents)
    }
    fn is_pid_valid(field_contents: &str) -> bool {
        static RE: OnceCell<Regex> = OnceCell::new();

        // Match only a 9 digit number including leading zeroes
        RE.get_or_init(|| Regex::new(r"^[0-9]{9}$").unwrap())
            .is_match(field_contents)
    }

    fn is_cid_valid(_field_contents: &str) -> bool {
        // ignore by default, so even if present just ignore
        true
    }
}

pub fn day04(input_lines: &str) -> (String, String) {
    // Passport candidates split by one blank line
    let re_empty_lines = Regex::new(r"\n\s*\n").unwrap();

    // Rust Set <==> HashSet, allowing us to use Boolean Operations
    let expected_pass_tags = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

    let mut valid_passports = 0;
    let mut truly_valid_passports = 0;

    for passport_candidate in re_empty_lines.split(input_lines) {
        let candidate = PassportCandidate::new(passport_candidate);

        if candidate.has_all_passport_fields(&expected_pass_tags) {
            valid_passports += 1;

            // expect valid passports to pass field checks before performing them
            truly_valid_passports += 1;

            // Need to check field validity for each truly valid PassportCandidate
            for tag in candidate.tag_set {
                let field_is_valid: bool = match &candidate.pass_fields.get_key_value(tag) {
                    Some((&"byr", field_contents)) => {
                        PassportCandidate::is_byr_valid(field_contents)
                    }
                    Some((&"iyr", field_contents)) => {
                        PassportCandidate::is_iyr_valid(field_contents)
                    }
                    Some((&"eyr", field_contents)) => {
                        PassportCandidate::is_eyr_valid(field_contents)
                    }
                    Some((&"hgt", field_contents)) => {
                        PassportCandidate::is_hgt_valid(field_contents)
                    }
                    Some((&"hcl", field_contents)) => {
                        PassportCandidate::is_hcl_valid(field_contents)
                    }
                    Some((&"ecl", field_contents)) => {
                        PassportCandidate::is_ecl_valid(field_contents)
                    }
                    Some((&"pid", field_contents)) => {
                        PassportCandidate::is_pid_valid(field_contents)
                    }
                    Some((&"cid", field_contents)) => {
                        PassportCandidate::is_cid_valid(field_contents)
                    }
                    Some((&&_, _)) => false,
                    None => false,
                };

                if !field_is_valid {
                    // passport is not truly valid if any field check fails
                    truly_valid_passports -= 1;
                    break;
                }
            }
        }
    }

    let answer1 = valid_passports;
    let answer2 = truly_valid_passports;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(
            day04(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm
        
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            )
            .0,
            "2".to_string()
        )
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(
            day04(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in

        eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007

        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            )
            .1,
            "6".to_string()
        )
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(day04(""), ("0".to_string(), "0".to_string()))
    }
}
