use std::fs;

pub fn get_pages_and_rules(file_path: &str) {
    let file = fs::read_to_string(&file_path).expect("File not found");

    // Pages and rules are separated by an empty line
    let (mut rules, mut pages) = file.split_once("\n\n").unwrap();

    // pages.iter().for_each(|line| {
    //     check_page_order(&rules, line);
    // })
    rules.lines().for_each(|rule| {});
    println!("Rules: {rules:?}");
    // println!("pages: {pages:?}");
}

// fn check_page_order(rules: &Vec<&str>, pages: &&str) {
//     pages.split(",").for_each(|page| {
//         check_against_rules(rules, page);
//     })
// }
//
// fn check_against_rules(rules: &Vec<&str>, page: &str) {
//     println!("Checking against pages: {}", page);
// }
