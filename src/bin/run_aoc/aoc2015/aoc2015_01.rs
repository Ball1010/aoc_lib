use aoc_lib::read_to_chars;


pub fn aoc2015_01(){
   //*Path from Crate level
   let ch = read_to_chars("src/inputs/2015_01.txt");
   //*
   let answer : i32 =ch.iter().map(|x| match x {
       '(' => 1 , 
       ')' => -1,
       _ => panic!("invalid character"),
   }).sum();
   println!("Day 1 pt 1: {answer}");

   let mut cur_floor = 1 ;
   for (pos , c) in ch.iter().enumerate() {
       let step = match c {
           '(' => 1 ,
           ')' => -1,
           _ => panic!("invalid character"),            
       };
       cur_floor += step;
       if cur_floor < 0 {
           println!("Day 1 pt 2: {pos}");
           break;
       }
    }
}