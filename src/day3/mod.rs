use regex::Regex;

pub fn clean_multiplication_file() -> i32{
    let input = include_str!("./inputs.txt");

    // parenthesis around \d indicate the groups we want to capture.
    let multi_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)");

    // Extract two digits from mul(n1,n2) statements and multiply them together.
    // Store result to perform sum later.
    multi_regex.unwrap().captures_iter(input).map(|c| {
        let n1 = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let n2 = c.get(2).unwrap().as_str().parse::<i32>().unwrap();

        n1 * n2
    }).sum()

}