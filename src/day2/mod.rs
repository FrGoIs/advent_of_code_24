pub fn get_reports_safety_count() -> i32 {
    let input = include_str!("./inputs.txt");
    let reports = input.lines().collect::<Vec<&str>>();
    let mut safety_count = 0;

    reports.into_iter().for_each(|report| {
        let mut levels = report.split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        // Could I make is_safe into an iterator?
        let is_safe  = is_safe(levels.clone()).0;
        if is_safe {
            safety_count += 1;
        }
        else {
            safety_count = if bad_problem_dampened( levels.clone()) { safety_count + 1 } else { safety_count };
        }
    });

    safety_count
}

fn is_safe(levels: Vec<i32> ) -> (bool, Vec<usize>) {
    let mut is_safe = true;
    let mut unsafe_indices: Vec<usize> = Vec::new();

    let is_increasing = if levels[0] < levels[1] { true } else { false };

    for i in 0..levels.len() - 1 {
        let difference = levels[i+1] - levels[i];
        if is_increasing && (difference > 3 || difference <= 0) {
            is_safe = false;
            unsafe_indices.push(i);
        }

        if !is_increasing && (difference < -3 || difference >= 0) {
            is_safe = false;
            unsafe_indices.push(i);
        }
    }

    (is_safe, unsafe_indices)
}



fn bad_problem_dampened(mut levels:  Vec<i32>) -> bool {
    // This is bad because it is brute force, but it is nice how easy it was to implement.
    for index in 0..levels.len(){
        let mut temp = levels.remove(index);
        if is_safe(levels.clone()).0 {
            return true;
        }
        else {
            levels.insert(index, temp);
        }
    }
    false
}