use std::str::FromStr ;
use std::fmt::Debug;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Point<T> {
    pub x: T ,
    pub y : T 
}
impl <T : FromStr>  Point<T> {
    pub fn parse(s:&str) -> Self
    where <T as FromStr>::Err : std::fmt::Debug {
      let mut value = s.split(',');
      let _x= value.next().unwrap().parse::<T>().unwrap();
      let _y= value.next().unwrap().parse::<T>().unwrap();
      Self { x : _x  , y : _y}
    }
 }
