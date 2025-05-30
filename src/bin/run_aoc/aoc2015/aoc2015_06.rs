use crate::Runner;
use aoc_lib::{read_lines , point::Point};
use std::str::FromStr;



#[derive(Debug)]
#[allow(dead_code)]
enum Command {
    Off(Point<usize> , Point<usize>),
    On(Point<usize> , Point<usize>),
    Toggle(Point<usize> , Point<usize>),
}

pub struct Aoc2015_06 {
    commands : Vec<Command>,
}

impl Aoc2015_06 {
   pub fn new()-> Self {
  let mut commands = Vec::new();     
      for line in read_lines("src/inputs/2015_06.txt") {
        let s: Vec<&str> = line.split(' ').collect();
        match s[1]  {
          "off" => {
          let start: Point<usize> = Point::parse(s[2]);
          let end: Point<usize> = Point::parse(s[4]);
          commands.push(Command::Off(start, end));
        }
          "on" => {
          let start: Point<usize> = Point::parse(s[2]);
          let end: Point<usize> = Point::parse(s[4]);
          commands.push(Command::On(start, end));
        }
        _ => {
            
          let start: Point<usize> = Point::parse(s[1]);
          let end: Point<usize> = Point::parse(s[3]);
          commands.push(Command::Toggle(start, end));
        }
      }
 
      }
      //println!("{:?}" , commands);

     Self { commands   }
    
   }
  }
impl Runner for Aoc2015_06 {
 fn name(&self) -> (usize,usize) {
      (2015 ,6)
 }

   fn part1(&mut self) -> Vec<String> {
      let mut grid =[[false ;1000];1000];
      for c in &self.commands {
        match c {
            Command::Off( p1, p2) =>{
              for gx in grid.iter_mut().take(p2.x +1).skip(p1.x) {
                for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y){
                  *gy =false;
                }
              }
            }
            Command::On( p1, p2) =>{
              for gx in grid.iter_mut().take(p2.x +1).skip(p1.x) {
                for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y){
                  *gy =true;
                }
              }
            }
            Command::Toggle( p1, p2) =>{
              for gx in grid.iter_mut().take(p2.x +1).skip(p1.x) {
                for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y){
                  *gy = !*gy;
                }
              }
            }
        }
      }
      let count1 = grid.iter().fold(0u32 , |a , b| a + b.iter().map(|x| *x as u32).sum::<u32>());
      vec![format!("{}" , count1)]
    }

    #[allow(static_mut_refs)]
   fn part2(&mut self) -> Vec<String> {
    unsafe {
    static mut GRID : [[u32 ;1000] ; 1000]=[[0u32 ;1000];1000];
    for c in &self.commands {
      match c {
          Command::Off( p1, p2) =>{
            for gx in GRID.iter_mut().take(p2.x +1).skip(p1.x) {
              for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y){
                *gy = gy.saturating_sub(1);
              }
            }
          }
          Command::On( p1, p2) =>{
            for gx in GRID.iter_mut().take(p2.x +1).skip(p1.x) {
              for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y){
                *gy += 1;
              }
            }
          }
          Command::Toggle( p1, p2) =>{
            for gx in GRID.iter_mut().take(p2.x +1).skip(p1.x) {
              for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y){
                *gy += 2;
              }
            }
            }
          }
      
  
   
    }
    let count1 = GRID.iter().fold(0u32, |a , b| a + b.iter().sum::<u32>());

    vec![format!("{}" , count1)]
  }
}
}
  
