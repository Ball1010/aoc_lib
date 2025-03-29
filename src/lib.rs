use std::{fmt::Debug, path::Path, str::FromStr};


pub fn read_to_chars<T : AsRef<Path>>(pathname : T) -> Vec<char> { 
    let data = std::fs::read_to_string(pathname).expect("Unable To Open File.");
    data.chars().collect()
}

pub fn numbers<T: AsRef<Path> , U : FromStr>(pathname : T , sep : char) -> Vec<Vec<U>>
where <U as FromStr>::Err: Debug{
    let data: String =  std::fs::read_to_string(pathname).expect("Unable To Open File.");
    let mut result: Vec<_> = Vec::new();
    for line in data.split("\n")  {
        if !line.is_empty(){
            let iter = line.split(sep);
            result.push(iter.map(|x| x.parse::<U>().expect("Error parsing")).collect::<Vec<U>>());
            };
    }
    result
}