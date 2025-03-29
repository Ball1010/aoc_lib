
use aoc_lib::read_to_chars;


pub struct Aoc2015_01 { 
    ch : Vec<char>, 

}
 impl  Aoc2015_01 {
    pub fn new() -> Self { 
        Self { ch: (read_to_chars("src/inputs/2015_01.txt")) }
    }
}
impl crate::Runner for Aoc2015_01 {
    fn part1(&mut self) -> Vec<String> {
        let answer : i32 =self.ch.iter().map(|x| match x {
            '(' => 1 , 
            ')' => -1,
            _ => panic!("invalid character"),
        }).sum();
        vec![format!("{answer}")]
    }

    fn part2(&mut self) -> Vec<String> {
        
   let mut cur_floor = 1 ;
   for (pos , c) in self.ch.iter().enumerate() {
       let step = match c {
           '(' => 1 ,
           ')' => -1,
           _ => panic!("invalid character"),            
       };
       cur_floor += step;
       if cur_floor < 0 {
        return vec![format!("{pos}")];
           
       }
    }
    panic!("No answer found")
}

    fn name(&self) -> (usize, usize) {
        (2015 ,1)
    }
}

