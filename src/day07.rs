use std::collections::HashMap;

use once_cell::sync::OnceCell;
use regex::Regex;

type Color = String;
#[derive(Debug)]
struct Contents {
    color: Color,
    count: usize,
}

#[derive(Debug)]
struct BagRules {
    color_to_contents_map: HashMap<Color, Vec<Contents>>,
}

fn bag_contains_gold(bag_rules: &BagRules, bag_contents: &Vec<Contents>) -> bool {
    for bag in bag_contents {
        if bag.color == "shiny gold" {
            return true;
        }
        if bag_contains_gold(
            bag_rules,
            bag_rules.color_to_contents_map.get(&bag.color).unwrap(),
        ) {
            return true;
        }
    }
    false
}

fn count_inside_bags(bag_rules: &BagRules, bag_contents: &Vec<Contents>) -> usize {
    let mut directly_contained_bags = 0;
    for bag in bag_contents {
        directly_contained_bags += bag.count
            * count_inside_bags(
                bag_rules,
                bag_rules.color_to_contents_map.get(&bag.color).unwrap(),
            );
    }
    directly_contained_bags += 1; //count yourself
    directly_contained_bags
}

impl BagRules {
    fn parse_rules(input: &str) -> BagRules {
        let mut color_to_contents_map: HashMap<Color, Vec<Contents>> = HashMap::new();

        for line in input.lines() {
            // sub-slice matching
            match &line.split(" bags contain ").collect::<Vec<_>>()[..] {
                &[parent_color, rules] => {
                    // bags that can't contain any further stored with empty vec
                    if rules == "no other bags." {
                        color_to_contents_map.insert(parent_color.to_string(), vec![])
                    } else {
                        color_to_contents_map.insert(
                            parent_color.to_string(),
                            BagRules::build_contents_vec(rules),
                        )
                    }
                }
                _ => panic!("Cannot parse rule: {}", line),
            };
        }
        BagRules {
            color_to_contents_map,
        }
    }

    fn build_contents_vec(rules: &str) -> Vec<Contents> {
        // use named capture groups using syntax (?P<name> regex)
        static BINDING: OnceCell<Regex> = OnceCell::new();
        let allowed_bags_parser =
            BINDING.get_or_init(|| Regex::new(r"(?P<count>\d) (?P<color>.+) bags?").unwrap());

        let mut contents_vec: Vec<Contents> = Vec::new();

        for rule in rules.split(", ") {
            let rule_contents = allowed_bags_parser.captures(rule).unwrap();

            // Extract named captures for count and color
            if let (Some(count), Some(color)) =
                (rule_contents.name("count"), rule_contents.name("color"))
            {
                // Contents objects including contained bag color and count of said bag
                let contents = Contents {
                    color: color.as_str().to_string(),
                    count: count.as_str().parse::<usize>().unwrap(),
                };

                contents_vec.push(contents);
            }
        }
        contents_vec
    }
}

pub fn day07(input_lines: &str) -> (String, String) {
    let bag_rules: BagRules = BagRules::parse_rules(input_lines);
    let mut ans1 = 0;

    for (_, bag_contents) in bag_rules.color_to_contents_map.iter() {
        if bag_contains_gold(&bag_rules, bag_contents) {
            ans1 += 1
        }
    }

    let gold_contents = bag_rules
        .color_to_contents_map
        .get(&"shiny gold".to_string())
        .unwrap();
    let mut ans2 = 0;

    for bag in gold_contents {
        ans2 += bag.count
            * count_inside_bags(
                &bag_rules,
                bag_rules.color_to_contents_map.get(&bag.color).unwrap(),
            )
    }

    (format!("{}", ans1), format!("{}", ans2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(
            day07(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            )
            .0,
            "4".to_string()
        )
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(
            day07(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            )
            .1,
            "32".to_string()
        )
    }

    #[test]
    fn check_day07_both_case1() {
        assert_eq!(
            day07(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            )
            .1,
            "126".to_string()
        )
    }
}
