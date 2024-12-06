use regex::Regex;

pub fn clean_multiplication_file() -> i32 {
    let input = include_str!("./inputs.txt");

    // parenthesis around \d indicate the groups we want to capture.
    let multi_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)");

    // Extract two digits from mul(n1,n2) statements and multiply them together.
    // Return the sum.
    multi_regex
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let (_, [n1, n2]) = c.extract();
            n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()
        })
        .sum()
}
