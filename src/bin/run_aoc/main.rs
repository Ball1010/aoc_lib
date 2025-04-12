#![allow(clippy::while_let_on_iterator)]

use std::time:: {Duration , Instant};

mod aoc2015;
use aoc2015::*;

pub enum Selector {
    All ,
    One(usize),
    Last
}

pub trait Runner  {
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
    fn name(&self) -> (usize , usize);
}

fn main(){
    //run_2015(Selector::All);
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1  {
        if let Ok(input) =args[1].parse::<usize>(){
        run_2015(Selector::One(input));
        } else {
        run_2015(Selector::All);
        }
    } else {
        run_2015(Selector::Last);
        }

}

fn run_solution<T:Runner + ?Sized> (solution : &mut T) {
    let nam: (usize, usize) = solution.name();
    println!("------ {} Day {} -----" ,nam.0 , nam.1);

    let start = Instant::now();
    let p1 = solution.part1();
    let p1_time =start.elapsed();
    print_solution(1, &p1 , p1_time);
    

    let start2 = Instant::now();
    let p2 = solution.part2();
    let p2_time =start2.elapsed();
    print_solution(2, &p2 , p2_time);
    
}



fn print_solution(which: usize , output: &[String] , dura : Duration) {
    let ms =dura.as_millis();
    let sec_part = ms / 1000;
    let ms_part = ms % 1000;
    let mut i = output.iter();
    println!("Part {which}: {} " , i.next().unwrap());
    while let Some(line) = i.next(){
        println!("       {line}");
    }
    println!("--Time: {sec_part:3}s {ms_part:03}ms\n");

}

