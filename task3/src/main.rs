use std::ops::Range;
use regex::Regex;


fn main() {
    let mut sum_parts = 0;
    let number_pattern = Regex::new(r"\d+").unwrap();
    let symbol_pattern = Regex::new(r"");

    let full_input = include_str!("../input.txt");
    let lines : Vec<&str> =full_input.lines().collect();
    let line_len = lines.get(0).unwrap().len()+1; //+1 as including \n, which is trimmed by lines()
    
    let all_ids = number_pattern.find_iter(full_input);
    let all_ids_g = number_pattern.find_iter(full_input);
    for id in all_ids {
        let range :Range<usize> = id.range();
        let mut is_part = false;
        for idx in range {
            let line = idx / line_len;
            let offset = idx % line_len;
            if num_is_part(&lines, line, offset) {
                is_part = true;
                break;
            }
        }
        if is_part {
            let part = id.as_str().parse::<i32>().unwrap();
            sum_parts += part;
            println!("Id: {part} is a part");
        }
    }
    println!("Total Sum of Part Numbers: {sum_parts}");
}

//Here we check if each adjacent spot is a symbol (other than .)
fn num_is_part(lines : &Vec<&str>, line : usize, offset : usize) -> bool {
    /*
     We will iterate TL->TR->L->R->BL->BR
     Theoretically we could try and load the top/bottom three chars in one go, but am unsure how this would work regarding our
     edge cases, so have opted to just do slightly slower.*/

    if line > 0{
        let top_line =  lines.get(line-1).unwrap().chars().collect::<Vec<char>>();
        if offset > 0 {
            let c = top_line.get(offset-1).unwrap();
            if  *c != '.' && c.is_ascii_punctuation() {
                return true;
            }
        }
        if let Some(c) = top_line.get(offset) {
            if  *c != '.' && c.is_ascii_punctuation() {
                return true;
            }
        } 
        if let Some(c) = top_line.get(offset+1) {
            if  *c != '.' && c.is_ascii_punctuation() {
                return true;
            }
        }
    }
    let same_line = lines.get(line).unwrap().chars().collect::<Vec<char>>();
    if offset > 0{
        let c = same_line.get(offset-1).unwrap();
        if  *c != '.' && c.is_ascii_punctuation() {
            return true;
        }
    }
    if let Some(c) = same_line.get(offset+1) {
        if  *c != '.' && c.is_ascii_punctuation() {
            return true;
        }
    }
    if let Some(bot_line) = lines.get(line+1) {
        let bot_line = bot_line.chars().collect::<Vec<char>>();
        if offset > 0 {
            let c = bot_line.get(offset-1).unwrap();
            if  *c != '.' && c.is_ascii_punctuation() {
                return true;
            }
        }
        if let Some(c) = bot_line.get(offset) {
            if  *c != '.' && c.is_ascii_punctuation() {
                return true;
            }
        }
        if let Some(c) = bot_line.get(offset+1) {
            if  *c != '.' && c.is_ascii_punctuation() {
                return true;
            }
        }
    }
    false
}

fn is_gear(lines : &Vec<&str>, idx : usize, line_len : usize) -> Option<i32> {
    let mut adjacent_parts = 0;
    


    return None;
}