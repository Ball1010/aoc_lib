use crate::Runner;


pub struct Aoc2015_04; 
#[allow(dead_code)]
impl Aoc2015_04 {
   pub fn new()-> Self {
     Self{}
    }
}
const PREFIX : &str = "iwrupvqb"; 


impl Runner for Aoc2015_04 {
 fn name(&self) -> (usize,usize) {
      (2015 ,4)
 }

   fn part1(&mut self) -> Vec<String> {
      let mut i =1;
      loop {
          let digest =md5::compute(format!("{PREFIX}{i}"));
          let string_digest = format!("{digest:x}");
          if string_digest.starts_with("00000"){
            return vec![format!("{i}")];
          }
          i +=1;
      }
      
    }

   fn part2(&mut self) -> Vec<String> {
    let mut i =1;
    loop {
        let digest =md5::compute(format!("{PREFIX}{i}"));
        let string_digest = format!("{digest:x}");
        if string_digest.starts_with("000000"){
          return vec![format!("{i}")];
        }
        i +=1;
    }
    
  }

}
