use regex::Regex;

fn main() {
    
    let lines : Vec<&str> = include_str!("../test.txt").lines().collect();
    let line_len = lines.get(0).unwrap().len();


}
