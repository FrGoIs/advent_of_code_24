use crate::day6::BoardState::{Free, Guard, Obstacle, Visited};
use crate::day6::GuardDirection::{East, North, South, West};
use crate::day6::SimulationState::Playable;
use std::fs;

const TEST_PATH: &str = "./src/day6/test.txt";
const INPUT_PATH: &str = "./src/day6/input.txt";

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum GuardDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum BoardState {
    Free,
    Guard(GuardDirection),
    Obstacle,
    Visited,
}
enum SimulationState {
    Playable,
    Finished,
}

type Board = Vec<Vec<BoardState>>;
fn calculate_guard_positions(input_path: &str) -> i32 {
    let input = fs::read_to_string(input_path).expect("Could not read file");
    let mut sim_state = Playable;
    let mut board = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Guard(North),
                    '>' => Guard(East),
                    '<' => Guard(West),
                    'v' => Guard(South),
                    '.' => Free,
                    '#' => Obstacle,
                    _ => unreachable!(),
                })
                .collect::<Vec<BoardState>>()
        })
        .collect::<Board>();


    while matches!(sim_state, Playable) {
        sim_state = simulate_step(&mut board);
    }
    count_visited_spaces(&board)
}
fn simulate_step(board: &mut Board ) -> SimulationState {
    let (mut guard_row, mut guard_col, mut dir): (usize, usize, GuardDirection) = (0, 0, North);
    let mut simulation_state = Playable;

    board.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, state)| {
            if matches!(state, Guard(_)) {
                // Get guard position and direction
                guard_row = x;
                guard_col = y;
                match state {
                    Guard(d) => dir = d.clone(),
                    _ => unreachable!(),
                }
            }
        })
    });

    if can_exit(board, guard_row, guard_col, dir) {
        return mark_finished(board, guard_row, guard_col);
    }

    if is_obstacle_ahead(board, guard_row, guard_col, dir) {
        return try_around_obstacle(board, guard_row, guard_col, dir);
    }
    // If cant exit nor is there an obstacle, move ahead.
    move_along_dir(board, guard_row, guard_col, dir);

    simulation_state
}

fn is_obstacle_ahead(board: &mut Board, row: usize, col: usize, dir: GuardDirection) -> bool {
    let state = match dir {
        North => {
            board[row - 1][col]
        }
        East => {
              board[row][col + 1]
        }
        South => {
             board[row + 1][col]
        }
        West => {
             board[row][col - 1]
        }
    };

    matches!(&state, Obstacle)
}

fn can_exit(board: &mut Board, row: usize, col: usize, dir: GuardDirection) -> bool {
    let last_row = board.len() - 1;
    let last_col = board[last_row].len() - 1;

    match dir {
        North => {row == 0},
        East => {col == last_col},
        South => {row == last_row},
        West => {col == 0 }
    }
}

fn mark_finished(board: &mut Board, guard_row: usize, guard_col: usize) -> SimulationState {
    board[guard_row][guard_col] = Visited;
    SimulationState::Finished
}

fn move_along_dir(board: &mut Board, guard_row: usize, guard_col: usize, dir: GuardDirection) {
    match dir {
        North => {
            board[guard_row - 1][guard_col] = Guard(North);
        }
        East => {
            board[guard_row][guard_col + 1] = Guard(East);
        }
        South => {
            board[guard_row + 1][guard_col] = Guard(South);

        }
        West => {
            board[guard_row][guard_col - 1] = Guard(West);
        }
    }
    board[guard_row][guard_col] = Visited;
}
fn try_around_obstacle(
    board: &mut Board,
    guard_row: usize,
    guard_col: usize,
    dir: GuardDirection,
) -> SimulationState {
    match dir {
        North => board[guard_row][guard_col] = Guard(East),
        East => board[guard_row][guard_col] = Guard(South),
        South => board[guard_row][guard_col] = Guard(West),
        West => board[guard_row][guard_col] = Guard(North),
    }

    simulate_step(board)
}
fn count_visited_spaces(board: &Board) -> i32 {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|board_state| {
                    if matches!(board_state, Visited | Guard(_)) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>() // Sum count per row
        })
        .sum::<i32>() // Sum over all rows
}

#[cfg(test)]
mod tests {
    use crate::day6::{calculate_guard_positions, INPUT_PATH, TEST_PATH};

    #[test]
    fn test_guard_positions() {
        assert_eq!(calculate_guard_positions(TEST_PATH), 41);
    }
    #[test]
    fn part1_distinct_positions() {
        assert_eq!(calculate_guard_positions(INPUT_PATH), 4758);
    }
    #[test]
    fn part2_count_loops() {

    }
}
