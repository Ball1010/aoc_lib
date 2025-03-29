use crate::Runner;
pub struct Aoc2015_04; 
impl Aoc2015_04 {
   pub fn new()-> Self {
     Self{}
    }
}

impl Runner for Aoc2015_04 {
 fn name(&self) -> (usize,usize) {
      (2015 ,4)
 }

   fn part1(&mut self) -> Vec<String> {
      vec!["unsolved".to_string()]
    }

   fn part2(&mut self) -> Vec<String> {
      vec!["unsolved".to_string()]
    }
}
