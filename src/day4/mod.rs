use std::fs;

const TEST_PATH: &str = "./src/day4/test.txt";
const INPUT_PATH: &str = "./src/day4/input.txt";

#[derive(Debug, PartialEq, Eq)]
enum SearchDir {
    N,
    E,
    S,
    W,
    NW,
    NE,
    SW,
    SE,
}

pub fn count_sequence_every_dir(input_path: &str, sequence: &str) -> i32 {
    let chars_to_find = sequence.chars().collect::<Vec<_>>();
    let input = fs::read_to_string(input_path).unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let col_count = matrix[0].len();
    let row_count = matrix.len();
    let mut matches = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, _character) in row.iter().enumerate() {
            // Get Applicable search directions
            let search_dirs = get_search_dirs(
                i as i32,
                j as i32,
                row_count as i32,
                col_count as i32,
                chars_to_find.len() as i32,
            );
            // Check if there are any matches along the available search directions starting from
            // the character at position (i,j)
            matches += check_for_sequence(search_dirs, i, j, &matrix, &chars_to_find);
        }
    }

    matches
}
// We are going to cheat a little bit and make this work only for sequences with 3 characters
// TODO: Extend it
pub fn count_crossed_sequence(input_path: &str, sequence: &str) -> i32 {
    let chars_to_find = sequence.chars().collect::<Vec<_>>();
    let input = fs::read_to_string(input_path).unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let col_count = matrix[0].len();
    let row_count = matrix.len();
    let mut matches = 0;

    // We will scan all rows and all columns
    for row in 0..row_count - 2 {
        for col in 0..col_count - 2 {
            // check middle character
            if matrix[row + 1][col + 1] == chars_to_find[1] {
                // from here there are only four possibilities
                if matrix[row][col] == matrix[row][col + 2] && matrix[row][col] == chars_to_find[0]
                {
                    if matrix[row + 2][col + 2] == matrix[row + 2][col]
                        && matrix[row + 2][col] == chars_to_find[2]
                    {
                        matches += 1;
                    }
                }
                if matrix[row + 2][col + 2] == matrix[row][col + 2]
                    && matrix[row][col + 2] == chars_to_find[0]
                {
                    if matrix[row][col] == matrix[row + 2][col]
                        && matrix[row][col] == chars_to_find[2]
                    {
                        matches += 1;
                    }
                }
                if matrix[row + 2][col] == matrix[row + 2][col + 2]
                    && matrix[row + 2][col] == chars_to_find[0]
                {
                    if matrix[row][col] == matrix[row][col + 2]
                        && matrix[row][col] == chars_to_find[2]
                    {
                        matches += 1;
                    }
                }
                if matrix[row][col] == matrix[row + 2][col] && matrix[row][col] == chars_to_find[0]
                {
                    if matrix[row + 2][col + 2] == matrix[row][col + 2]
                        && matrix[row + 2][col + 2] == chars_to_find[2]
                    {
                        matches += 1;
                    }
                }
            }
        }
    }
    matches
}

fn check_for_sequence(
    dirs: Vec<SearchDir>,
    row: usize,
    col: usize,
    matrix: &Vec<Vec<char>>,
    chars_to_find: &Vec<char>,
) -> i32 {
    let mut matches = 0;

    dirs.iter().for_each(|dir| {
        let mut walking_index = 0;
        match *dir {
            SearchDir::N => {
                while matrix[row - walking_index][col] == chars_to_find[walking_index] {
                    walking_index += 1;
                    if walking_index == chars_to_find.len() {
                        break;
                    }
                }
            }
            SearchDir::E => {
                while matrix[row][col + walking_index] == chars_to_find[walking_index] {
                    walking_index += 1;
                    if walking_index == chars_to_find.len() {
                        break;
                    }
                }
            }
            SearchDir::S => {
                while matrix[row + walking_index][col] == chars_to_find[walking_index] {
                    walking_index += 1;
                    if walking_index == chars_to_find.len() {
                        break;
                    }
                }
            }
            SearchDir::W => {
                while matrix[row][col - walking_index] == chars_to_find[walking_index] {
                    walking_index += 1;
                    if walking_index == chars_to_find.len() {
                        break;
                    }
                }
            }
            SearchDir::NW => {
                while matrix[row - walking_index][col - walking_index]
                    == chars_to_find[walking_index]
                {
                    walking_index += 1;
                    if walking_index == chars_to_find.len() {
                        break;
                    }
                }
            }
            SearchDir::NE => {
                while matrix[row - walking_index][col + walking_index]
                    == chars_to_find[walking_index]
                {
                    walking_index += 1;
                    if walking_index == chars_to_find.len() {
                        break;
                    }
                }
            }
            SearchDir::SW => {
                while matrix[row + walking_index][col - walking_index]
                    == chars_to_find[walking_index]
                {
                    walking_index += 1;
                    if walking_index == chars_to_find.len() {
                        break;
                    }
                }
            }
            SearchDir::SE => {
                while matrix[row + walking_index][col + walking_index]
                    == chars_to_find[walking_index]
                {
                    walking_index += 1;
                    if walking_index == chars_to_find.len() {
                        break;
                    }
                }
            }
        }
        if walking_index == chars_to_find.len() {
            matches += 1;
        }
    });
    matches
}

fn get_search_dirs(
    row: i32,
    col: i32,
    row_count: i32,
    col_count: i32,
    sequence_length: i32,
) -> Vec<SearchDir> {
    let mut search_dirs: Vec<SearchDir> = Vec::new();

    // Can go up
    if row - sequence_length + 1 >= 0 {
        search_dirs.push(SearchDir::N);
    }
    // Can go left
    if col - sequence_length + 1 >= 0 {
        search_dirs.push(SearchDir::W);
    }
    // can go right
    if col_count - col - sequence_length >= 0 {
        search_dirs.push(SearchDir::E);
    }
    // Can go down
    if row_count - row - sequence_length >= 0 {
        search_dirs.push(SearchDir::S);
    }

    if search_dirs.contains(&SearchDir::N) && search_dirs.contains(&SearchDir::E) {
        search_dirs.push(SearchDir::NE);
    }
    if search_dirs.contains(&SearchDir::N) && search_dirs.contains(&SearchDir::W) {
        search_dirs.push(SearchDir::NW);
    }
    if search_dirs.contains(&SearchDir::S) && search_dirs.contains(&SearchDir::E) {
        search_dirs.push(SearchDir::SE);
    }
    if search_dirs.contains(&SearchDir::S) && search_dirs.contains(&SearchDir::W) {
        search_dirs.push(SearchDir::SW);
    }

    search_dirs
}

#[cfg(test)]
mod day4_tests {
    use crate::day4::*;

    #[test]
    fn finds_test_xmas() {
        let result = count_sequence_every_dir(TEST_PATH, "XMAS");
        assert_eq!(result, 18);
    }
    #[test]
    fn finds_xmas_every_dir() {
        let result = count_sequence_every_dir(INPUT_PATH, "XMAS");
        assert_eq!(result, 2543);
    }

    #[test]
    fn finds_crossed_sequence() {
        assert_eq!(count_crossed_sequence(INPUT_PATH, "MAS"), 1930);
    }
}
