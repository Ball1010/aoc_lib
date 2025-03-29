mod aoc2015_02;
use aoc2015_01::Aoc2015_01;
use aoc2015_02::Aoc2015_02;



use crate::run_solution;

pub mod aoc2015_01;


pub fn run_2015(){
    let mut day01 = Aoc2015_01::new();
    run_solution(&mut day01);

    let mut day02 = Aoc2015_02::new();
    run_solution(&mut day02);
}