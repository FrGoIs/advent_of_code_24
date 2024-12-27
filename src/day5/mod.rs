use std::collections::HashMap;
use std::fs;
const TEST_PATH: &str = "./src/day5/test.txt";
const INPUT_PATH: &str = "./src/day5/input.txt";

type RulesMap = HashMap<String, Vec<String>>;
type PageOrderings = Vec<Vec<String>>;
pub fn sum_correct_middle_pages(input_path: &str) -> i32 {
    let (rules_map, orderings) = parse_rules_and_orderings(input_path);

    orderings
        .iter()
        .map(|pages_list| check_valid_pages_list(rules_map.clone(), pages_list))
        .sum()
}

pub fn sum_incorrect_middle_pages(input_path: &str) -> i32 {
    let (rules_map, orderings) = parse_rules_and_orderings(input_path);

    orderings
        .iter()
        .filter(|pages_list| !is_ordering_valid(rules_map.clone(), pages_list))
        .map(|pages_list| {
            let fixed = fix_page_orderings(rules_map.clone(), pages_list.clone());
            println!("Fixed: {fixed:?}");
            check_valid_pages_list(rules_map.clone(), &fixed)
        })
        .sum()
}

fn fix_page_orderings(rules: RulesMap, pages_list: Vec<String>) -> Vec<String> {
    let mut fixed: Vec<String> = Vec::new();

    for (index, page) in pages_list.iter().enumerate() {
        if index == 0 {
            fixed.push(pages_list[0].clone())
        }

        for i in 0..index {
            match rules.get(page) {
                Some(pages) => {
                    if pages.contains(&pages_list[i].to_string()) {
                        fixed.insert(i, pages_list[index].to_string());
                        break;
                    }
                }
                None => {
                    fixed.push(pages_list[index].to_string());
                    break;
                }
            }
            if i == index - 1 {
                fixed.push(page.clone());
            }
        }
    }
    if !is_ordering_valid(rules.clone(), &fixed) {
        fixed = fix_page_orderings(rules, fixed);
    }
    fixed
}

fn is_ordering_valid(rules: RulesMap, pages_list: &Vec<String>) -> bool {
    let mut is_valid = true;
    for (index, page) in pages_list.iter().enumerate() {
        for i in 0..index {
            match rules.get(page) {
                Some(pages) => {
                    if pages.contains(&pages_list[i].to_string()) {
                        is_valid = false;
                        break;
                    }
                }
                None => {}
            }
        }
    }
    is_valid
}
fn check_valid_pages_list(rules: RulesMap, pages_list: &Vec<String>) -> i32 {
    let is_valid = is_ordering_valid(rules, pages_list);

    if is_valid {
        let middle_page = pages_list[(pages_list.len()) / 2].clone();
        return middle_page.parse::<i32>().unwrap();
    }

    0
}

fn parse_rules_and_orderings(input_path: &str) -> (RulesMap, PageOrderings) {
    let input = fs::read_to_string(input_path).expect("Could not read input file");
    let (rules_list, page_orderings) = input.split_once("\n\n").expect("bad input");

    let mut rules_map: RulesMap = RulesMap::new();

    rules_list.lines().for_each(|rule| {
        let rule_pairs = rule
            .split("|")
            .map(|r| r.to_string())
            .collect::<Vec<String>>();

        rules_map
            .entry(rule_pairs[0].clone())
            .or_insert(Vec::new())
            .push(rule_pairs[1].clone());
    });

    let orderings = page_orderings
        .lines()
        .map(|o| {
            o.split(",")
                .into_iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<PageOrderings>();

    (rules_map, orderings)
}
#[cfg(test)]
mod tests {
    use crate::day5::{
        fix_page_orderings, is_ordering_valid, parse_rules_and_orderings, sum_correct_middle_pages,
        sum_incorrect_middle_pages, INPUT_PATH, TEST_PATH,
    };

    #[test]
    fn sum_middle_pages() {
        assert_eq!(sum_correct_middle_pages(TEST_PATH), 143);
    }

    #[test]
    fn part_one() {
        assert_eq!(sum_correct_middle_pages(INPUT_PATH), 5747);
    }
    #[test]
    fn part_two() {
        assert_eq!(sum_incorrect_middle_pages(INPUT_PATH), 5502);
    }
    #[test]
    fn should_return_correct_orderings() {
        let rules_map = parse_rules_and_orderings(TEST_PATH).0;
        let orderings: Vec<String> = vec![
            "97".to_string(),
            "13".to_string(),
            "75".to_string(),
            "29".to_string(),
            "47".to_string(),
        ];

        let fixed = fix_page_orderings(rules_map.clone(), orderings.clone());
        assert_eq!(is_ordering_valid(rules_map.clone(), &fixed), true);
        assert_eq!(fixed.len(), orderings.len());
    }
}
