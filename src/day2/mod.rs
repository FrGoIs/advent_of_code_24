pub fn get_reports_safety_count() -> i32 {
    let input = include_str!("input.txt");
    let reports = input.lines().collect::<Vec<&str>>();
    let mut safety_count = 0;

    reports.into_iter().for_each(|report| {
        let levels = report
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&levels) {
            safety_count += 1;
        } else {
            // If we don't want to modify levels by accident,
            // we pass a copy instead of the reference
            safety_count = if bad_problem_dampened(levels.clone()) {
                safety_count + 1
            } else {
                safety_count
            };
        }
    });

    safety_count
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut is_safe = true;

    let is_increasing = if levels[0] < levels[1] { true } else { false };

    for i in 0..levels.len() - 1 {
        let difference = levels[i + 1] - levels[i];
        if is_increasing && (difference > 3 || difference <= 0) {
            is_safe = false;
        }

        if !is_increasing && (difference < -3 || difference >= 0) {
            is_safe = false;
        }
    }

    is_safe
}

fn bad_problem_dampened(mut levels: Vec<i32>) -> bool {
    // This is bad because it is brute force,
    // but it is nice how easy it was to implement.
    for index in 0..levels.len() {
        let temp = levels.remove(index);
        if is_safe(&levels) {
            return true;
        } else {
            levels.insert(index, temp);
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::day2::*;
    #[test]
    fn dampened_problem_test() {
        assert_eq!(561, get_reports_safety_count());
    }
}
