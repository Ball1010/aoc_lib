#![allow(clippy::while_let_on_iterator)]

mod aoc2015;
use aoc2015::*;


pub trait Runner  {
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
    fn name(&self) -> (usize , usize);
}


fn main(){
    run_2015();

}

fn run_solution<T:Runner> (solution : &mut T) {
    let nam = solution.name();
    println!("------ {} Day {} -----" ,nam.0 , nam.1);

    print_solution(1, &solution.part1());
    print_solution(2, &solution.part2());
    
}


fn print_solution(which: usize , output: &[String]) {
    let mut i = output.iter();
    println!("Part {which}: {} " , i.next().unwrap());
    while let Some(line) = i.next(){
        println!("       {line}");
    }
}