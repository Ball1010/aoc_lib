use aoc_lib::numbers;

use crate::Runner;

pub struct Present([i32 ; 3]);
impl Present {
    fn new (v: &[i32]) -> Self{
        Self([v[0] ,v[1] , v[2]] )
    }
    fn suface_area (&self) -> i32 {
        2 * self.0[0] *self.0[1] + 2* self.0[0] * self.0[2] +self.0[1] *2 *self.0[2]
    } 
    fn slack (&self)-> i32 {
        self.0[0] * self.0[1] 
    }
    fn ribbon (&self) -> i32 {
        2*self.0[0] + 2*self.0[1] + self.0[0] *self.0[1]*self.0[2]
    }
}
pub struct Aoc2015_02 {
    prez : Vec<Present>
}

impl Aoc2015_02 {
    pub fn new() -> Self{
       let mut data =numbers("src/inputs/2015_02.txt" , 'x');
       let mut prez = Vec::new();
       for d in &mut data  {
            d.sort();
           prez.push(Present::new(d))
       }
       Self { prez}
    }
}

impl Runner for Aoc2015_02 {
    fn name(&self) -> String {
        " 2015 DAY 2 ".to_string()
    }
    fn part1(&mut self) -> Vec<String> {
       vec![format!("{}" , self.prez.iter().map(|p| p.suface_area() + p.slack()).sum::<i32>())]
        
    }
    fn part2(&mut self) -> Vec<String> {
        vec![format!("{}" , self.prez.iter().map(|p| p.ribbon()).sum::<i32>())]
    }
}