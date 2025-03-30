use std::{collections::HashSet, fmt::format, iter::zip};

use crate::Runner;
use aoc_lib::read_lines;
pub struct Aoc2015_05{
  data : Vec<String>,
}

impl Aoc2015_05 {
   pub fn new()-> Self {
     Self{ data: read_lines("src/inputs/2015_05.txt")}
    }
}

impl Runner for Aoc2015_05 {
 fn name(&self) -> (usize,usize) {
      (2015 ,5)
 }

   fn part1(&mut self) -> Vec<String> {
    let mut nice_words_1 : usize = 0
  ;
    for x in &self.data {
     let vowels =  x.chars().filter(|p| matches!(p, 'a' | 'e' | 'i' | 'o' | 'u')).count();
     
     if vowels < 3 {
      continue;
    }
    let mut found: bool = false;
    for pair in zip(x.chars(), x.chars().skip(1)){
      if pair.0 ==pair.1 {
        found = true;
        break;
      }
    }
    if !found {
      continue;
    }
    if x.contains("ab") {
        continue;
    }
    if x.contains("cd") {
      continue;
    }
    if x.contains("pq") {
    continue;
    }
    if x.contains("xy") {
      continue;
    }

    nice_words_1 +=1;

    }
    

      vec![format!("{nice_words_1}")]
}

   fn part2(&mut self) -> Vec<String> {
    {
      let mut nice_words_1 : usize = 0;
      for x in &self.data {

        let mut found: bool = false;

        for pair in zip(x.chars(), x.chars().skip(1)).enumerate()  {
            let fstring = format!("{}{}" , pair.1.0 ,pair.1.1);
            let f = fstring.as_str();
            if let Some(index) = x.rfind(f){
              if index > pair.0 +1 {
                  found = true;
                  break;
              }
            }
        }


        if !found {
          continue;
        }
        found =false;
        for pair in zip(x.chars(), x.chars().skip(2)){
          if pair.0 ==pair.1 {
            found = true;
            break;
          }
        }
        if found {
        nice_words_1 +=1;
         }
      }
      
  
        vec![format!("{nice_words_1}")]
    }
  }
}