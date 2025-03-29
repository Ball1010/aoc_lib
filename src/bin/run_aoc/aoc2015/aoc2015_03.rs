use std::collections::HashSet;

use crate::Runner;
use aoc_lib::read_to_chars;

pub struct Aoc2015_03{
    data : Vec<char>,
}


impl Aoc2015_03  {
    pub fn new ()-> Self {
        Self{ data : read_to_chars("src/inputs/2015_03.txt")}
        
    }
}

impl Runner for Aoc2015_03 {
    fn name(&self) -> (usize , usize) {
        (2015 ,3)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid: HashSet<(i32, i32)> = HashSet::new();
        let mut x: i32 =0;
        let mut y: i32 =0;

        for g in &self.data { 
            grid.insert((x,y));
            match g {
                '^' => y +=1,
                'v' => y -=1,
                '>' => x +=1,
                '<' => x -=1,
                _ =>panic!("bad char input"),
            }
        }
        grid.insert((x,y));

        vec![format!("{}" , grid.len())]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut grid = HashSet::new();
        let mut x =[0,0];
        let mut y=[0,0];
        let mut which = 0;

        for g in &self.data { 
            grid.insert((x[which],y[which]));
            match g {
                '^' => y[which] +=1,
                'v' => y[which] -=1,
                '>' => x[which] +=1,
                '<' => x[which] -=1,
                _ =>panic!("bad char input"),
            }
            which = 1 -which;
        }
        grid.insert((x[which],y[which]));

        vec![format!("{}" , grid.len())]
    }
}

