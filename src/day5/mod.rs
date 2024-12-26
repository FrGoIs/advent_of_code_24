use std::collections::HashMap;
use std::fs;
const TEST_PATH: &str = "./src/day5/test.txt";
const INPUT_PATH: &str = "./src/day5/input.txt";
pub fn sum_correct_middle_pages(input_path: &str) -> i32 {
    let input = fs::read_to_string(input_path).expect("Could not read input file");
    let mut rules_map: HashMap<String, Vec<String>> = HashMap::new();
    let (rules_list, page_orderings) = input.split_once("\n\n").expect("bad input");

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

    let order = page_orderings
        .lines()
        .map(|o| o.split(",").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    order
        .iter()
        .map(|pages_list| check_pages_list(rules_map.clone(), pages_list))
        .sum()
}

fn check_pages_list(rules: HashMap<String, Vec<String>>, pages_list: &Vec<&str>) -> i32 {
    let mut is_valid = true;
    for (index, page) in pages_list.iter().enumerate() {
        for i in (0..index) {
            match rules.get(page.clone()) {
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
    if is_valid {
        let middle_page = pages_list[(pages_list.len()) / 2];
        return middle_page.parse::<i32>().unwrap();
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day5::{sum_correct_middle_pages, INPUT_PATH, TEST_PATH};

    #[test]
    fn sum_middle_pages() {
        assert_eq!(sum_correct_middle_pages(TEST_PATH), 143);
    }

    #[test]
    fn part_one() {
        assert_eq!(sum_correct_middle_pages(INPUT_PATH), 5747);
    }
}
