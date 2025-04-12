
use std::{cell::RefCell, collections::HashMap };

use crate::Runner;
use aoc_lib::read_lines;

#[derive(Debug , PartialEq )]
pub enum Term {
    Var(String),
    Number(u16),
}
impl From<String> for Term {
    fn from(s : String) -> Self { 
        if let Ok(numb) = s.parse::<u16>() {
            Term::Number(numb)
        }else {
            Term::Var(s)
        }
    }
}

 impl From<&String> for Term {
    fn from(s : &String) -> Self { 
        if let Ok(numb) = s.parse::<u16>() {
            Term::Number(numb)
        }else {
            Term::Var(s.clone())
        }
    }
}



#[derive(Debug , PartialEq)]
pub enum Opt {
    Num(u16),
    Not(String),
    Assign(String),
    And(Term , Term),
    Or (Term , Term),
    Lshift ( String , u16),
    Rshift( String , u16),

}

pub struct Aoc2015_07 { 
    commands : HashMap< String , Opt>, 
    wire : RefCell<HashMap<String, u16>>,
    answer1 : u16 
} 


impl Aoc2015_07 {
   pub fn new()-> Self {
        Self { commands: Self::load(&read_lines("src/inputs/2015_07.txt")),
             wire: RefCell::new(HashMap::new()),
             answer1 :0 }
    }
    fn load(lines : &[String]) -> HashMap<String , Opt> {
        let mut commands: HashMap<String, Opt> = HashMap::new();
        for line in lines {
            let (inp , out) = line.split_once(" -> ").unwrap();
            let com = inp.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
            let command = 
            
            if com.len() == 1 && com[0].chars().next().unwrap().is_numeric() {
                Opt::Num(com[0].parse::<u16>().unwrap())
            } else if com.len()== 1 {
                Opt::Assign(com[0].clone())
            } else if com[0] == "NOT"{
                Opt::Not(com[1].clone())
            } else {
                match com[1].as_str() {
                    "RSHIFT" => Opt::Rshift(com[0].clone(), com[2].parse::<u16>().unwrap()),
                    "LSHIFT" => Opt::Lshift(com[0].clone(), com[2].parse::<u16>().unwrap()),
                    "AND" => Opt::And(com[0].clone().into(), com[2].clone().into()),
                    "OR" => Opt::Or(com[0].clone().into(), com[2].clone().into()),
                    _ => panic!("Invalid Input"),
                }
                
            };
            commands.insert(out.to_string(),command);
           // println!("{:?}" , commands)
        }
         commands
    }

    fn get_value(&self , var : &Term) -> u16 { 
        let marb_  = match var {
            Term::Number(n) => return *n,
            Term::Var(v) =>  v,
        };
        if let Some(value) = self.wire.borrow().get(marb_) {
            return *value;
        }
        
        let command = self.commands.get(marb_).unwrap();
       // println!("{command:?}");
        let value = match command {
            Opt::Num(val) => *val,
            Opt::Assign(name) =>self.get_value(&name.into()),
            Opt::And(left, right ) => {
                let left = self.get_value(left);
                let right = self.get_value(right);
                left & right
            }
            Opt::Or(left, right ) => {
                let left = self.get_value(left);
                let right = self.get_value(right);
                left | right
            }
            Opt::Lshift(left , amt) => {
                let left = left.clone();
                let amt =*amt;
                let left = self.get_value(&left.into());
                left << amt
            }
            Opt::Rshift(right , amt) => {
                let right = right.clone();
                let amt =*amt;
                let right = self.get_value(&right.into());
                right >> amt
            }
            Opt::Not(name) =>{
                let name =self.get_value(&name.into());
                !name
            }
        };
        self.wire.borrow_mut().insert(marb_.clone(), value);
        //println!("{value}");
        value
        
    }      

    fn set_b(&self , val : u16) {
        self.wire.borrow_mut().clear();
        self.wire.borrow_mut().insert("b".to_string(), val);
    }

    
     
}



impl Runner for Aoc2015_07 {
 fn name(&self) -> (usize,usize) {
      (2015 ,7)
 }

   fn part1(&mut self) -> Vec<String> {
    let answer1 =self.get_value(&"a".to_string().into());
    vec![format!("{}" , answer1 )]
    }

   fn part2(&mut self) -> Vec<String> {
    self.answer1 = self.get_value(&"a".to_string().into());
    self.set_b(self.answer1);
    vec![format!("{}" , self.get_value(&"a".to_string().into()) )]
    }
}
