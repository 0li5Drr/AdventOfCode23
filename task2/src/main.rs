use regex::Regex;


const MAX_RED   :i32 = 12;
const MAX_GREEN :i32 = 13;
const MAX_BLUE  :i32 = 14;
fn main() {
    let lines : Vec<&str> = include_str!("../input.txt").lines().collect();

    let  reg_r = Regex::new(r"red").unwrap();
    let  reg_g = Regex::new(r"green").unwrap();
    let  reg_b = Regex::new(r"blue").unwrap();
    let number = Regex::new(r"\d+").unwrap();

    let mut res : i32 = 0;
    let mut power :i32 = 0;

    

    for i in 1..=lines.len() {
        let line = lines.get(i-1).unwrap();
        let (_, game) = line.split_once(':').unwrap();
        print!("Game {i}: {game:#?}\t");
        let sets : Vec<&str> = game.split(';').collect();
        let mut most_red    : i32 = 0;
        let mut most_green  : i32 = 0;
        let mut most_blue   : i32 = 0;

        //Now we parse each set to add to our current green red and blue
        for set in sets {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for pull in set.split(',') {
                if reg_r.is_match(pull) {
                    let r  = number.find(pull).unwrap().as_str().parse::<i32>().unwrap();
                    red += r;
                    print!("r: {r:?}, ");
                } else if reg_b.is_match(pull) {

                    let b  = number.find(pull).unwrap().as_str().parse::<i32>().unwrap();
                    blue += b;
                    print!("b: {b:?}, ");
                } else if reg_g.is_match(pull) {
                    let g = number.find(pull).unwrap();
                    let g  = g.as_str().parse::<i32>().unwrap();
                    green += g;
                    print!("g: {g:?}, ");
                }
            }
            print!(";");
            most_red = most_red.max(red);
            most_blue = most_blue.max(blue);
            most_green =  most_green.max(green);
        } // Set Loop
        print!("\n");
        if most_red <= MAX_RED && most_blue <= MAX_BLUE && most_green <= MAX_GREEN {
            println!("⭐ Game {i} possible, with {most_red} Red, {most_green} Green and {most_blue} Blue.");
            res += i as i32;
        }
        let game_pow = most_red * most_blue * most_green;
        power += game_pow;
        println!("🔥 Game {i} had a power of {game_pow}, bring the total to: {power}");
    } // Game Loop
    println!("Sum of ID of all possible games: {res}");

}
