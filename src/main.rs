use crate::day1::{get_distance_and_similarity };
use crate::day2::get_reports_safety_count;

mod day1;
mod day2;

fn main() {
    println!("Day 1: {:?}", get_distance_and_similarity());
    println!("Day 2: {:?}", get_reports_safety_count());
}
