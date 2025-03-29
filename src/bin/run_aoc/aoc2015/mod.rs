#![allow(unused_imports)]
mod aoc2015_03;
mod aoc2015_02;
mod aoc2015_01;
mod aoc2015_04;
use aoc2015_01::*;
use aoc2015_02::*;
use aoc2015_03::*;
use aoc2015_04::*;


use crate::run_solution;




pub fn run_2015(){
    let mut day01 = Aoc2015_01::new();
    run_solution(&mut day01);

    let mut day02 = Aoc2015_02::new();
    run_solution(&mut day02);

    let mut day03 = Aoc2015_03::new();
    run_solution(&mut day03);

    println!("------ 2015 Day 4 ------\n       -Skipped- \n")
    //let mut day04 = Aoc2015_04::new();
    //run_solution(&mut day04);

    
}