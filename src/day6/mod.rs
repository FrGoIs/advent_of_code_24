use crate::day6::BoardStates::{Free, Guard, Obstacle, Visited};
use crate::day6::GuardDirection::{East, North, South, West};
use crate::day6::SimulationState::Playable;
use std::fs;

const TEST_PATH: &str = "./src/day6/test.txt";
const INPUT_PATH: &str = "./src/day6/input.txt";

#[derive(Debug, Eq, PartialEq, Clone)]
enum GuardDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq)]
enum BoardStates {
    Free,
    Guard(GuardDirection),
    Obstacle,
    Visited,
}
enum SimulationState {
    Playable,
    Finished,
}

type Board = Vec<Vec<BoardStates>>;
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
                .collect::<Vec<BoardStates>>()
        })
        .collect::<Board>();

    let last_row = board.len() - 1;
    let last_col = board[0].len() - 1;

    while matches!(sim_state, Playable) {
        sim_state = simulate_step(&mut board, last_row, last_col);
    }
    count_visited_spaces(&board)
}
fn simulate_step(board: &mut Board, last_row: usize, last_col: usize) -> SimulationState {
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

    match dir {
        North => {
            // Guard Exits through the top
            if guard_row == 0 {
                return mark_finished(board, guard_row, guard_col);
            }
            // If guard cant exist we need to look for where it moves
            match board[guard_row - 1][guard_col] {
                Obstacle => {
                    return try_around_obstacle(board, guard_row, guard_col, North);
                }
                _ => {
                    board[guard_row - 1][guard_col] = Guard(North);
                    board[guard_row][guard_col] = Visited;
                }
            }
        }
        East => {
            if guard_col == last_col {
                return mark_finished(board, guard_row, guard_col);
            }
            match board[guard_row][guard_col + 1] {
                Obstacle => {
                    return try_around_obstacle(board, guard_row, guard_col, East);
                }
                _ => {
                    board[guard_row][guard_col + 1] = Guard(East);
                    board[guard_row][guard_col] = Visited;
                }
            }
        }
        South => {
            if guard_row == last_row {
                return mark_finished(board, guard_row, guard_col);
            }
            match board[guard_row + 1][guard_col] {
                Obstacle => {
                    return try_around_obstacle(board, guard_row, guard_col, South);
                }
                _ => {
                    board[guard_row + 1][guard_col] = Guard(South);
                    board[guard_row][guard_col] = Visited;
                }
            }
        }
        West => {
            if guard_col == 0 {
                return mark_finished(board, guard_row, guard_col);
            }
            match board[guard_row][guard_col - 1] {
                Obstacle => {
                    return try_around_obstacle(board, guard_row, guard_col, West);
                }
                _ => {
                    board[guard_row][guard_col - 1] = Guard(West);
                    board[guard_row][guard_col] = Visited;
                }
            }
        }
    }
    simulation_state
}

fn mark_finished(board: &mut Board, guard_row: usize, guard_col: usize) -> SimulationState {
    board[guard_row][guard_col] = Visited;
    SimulationState::Finished
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

    simulate_step(board, board.len() - 1, board[0].len() - 1)
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
        assert_eq!(calculate_guard_positions(TEST_PATH), 38);
    }
    #[test]
    fn part1_distinct_positions() {
        assert_eq!(calculate_guard_positions(INPUT_PATH), 4758);
    }
    #[test]
    fn part2_count_loops() {

    }
}
