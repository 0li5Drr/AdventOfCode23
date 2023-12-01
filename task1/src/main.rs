

fn main() {
    let lines : Vec<&str> = include_str!("../input.txt").lines().collect();
    let res1 = lines.iter().fold(0, |acc, l| {acc + work1( *l)});
    let res2 = lines.iter().fold(0, |acc, l| acc + work2(*l));
    println!("The Sum of the hidden parameters is: {res1}, {res2}");
}



fn work1( line : &str) -> i32 {
    let mut num1 : i32=-1;
    let mut num2 : i32=-1;

    //Get First Number
    for c  in line.as_bytes().iter() {
        match *c {
            0x30 => num1 = 0,
            0x31 => num1 = 10,
            0x32 => num1 = 20,
            0x33 => num1 = 30,
            0x34 => num1 = 40,
            0x35 => num1 = 50,
            0x36 => num1 = 60,
            0x37 => num1 = 70,
            0x38 => num1 = 80,
            0x39 => num1= 90,
            _ => {continue}
        }
        if num1 >= 0 {
            break;
        }
    }
    //Get Second Number 
    for c in line.as_bytes().iter().rev() {
        match *c {
            0x30 => num2 = 0,
            0x31 => num2 = 1,
            0x32 => num2 = 2,
            0x33 => num2= 3,
            0x34 => num2 = 4,
            0x35 => num2 = 5,
            0x36 => num2 = 6,
            0x37 => num2 = 7,
            0x38 => num2 = 8,
            0x39 => num2 = 9,
            _ => {continue}
        }
        if num2 >= 0 {
            break;
        }
    }
    num1 + num2
}


fn safe_rev(bytes : &[u8]) -> Vec<u8> {
    let mut rev :Vec<u8> = vec![];
    for b in bytes {
        rev.push(*b);
    }
    // print!("Bytes: {bytes:?}");
    rev.reverse();
    // print!("Rev: {rev:?}\n");
    rev

}

fn work2(line : &str) -> i32 {
    let mut num1 :i32 = -1;
    let mut num2 = -1;

    let ONE   : &mut[u8] = &mut [0x6f, 0x6e, 0x65];
    let TWO   : &mut[u8] = &mut [0x74, 0x77 ,0x6f];
    let THREE : &mut[u8] = &mut [0x74, 0x68, 0x72, 0x65, 0x65];
    let FOUR  : &mut[u8] = &mut [0x66, 0x6f, 0x75, 0x72];
    let FIVE  : &mut[u8] = &mut [0x66, 0x69, 0x76, 0x65];
    let SIX   : &mut[u8] = &mut [0x73, 0x69, 0x78];
    let SEVEN : &mut[u8] = &mut [0x73, 0x65, 0x76, 0x65, 0x6e];
    let EIGHT : &mut[u8] = &mut [0x65, 0x69, 0x67, 0x68, 0x74];
    let NINE  : &mut[u8] = &mut [0x6e, 0x69, 0x6e, 0x65];

    let line = line.to_ascii_lowercase();
    print!("{line}: ");
    let bytes = line.as_bytes();
    let len = bytes.len();
    //First Num: 
    for i in 0..bytes.len() {
        if let Some(c) = bytes.get(i) {
            if *c <= 0x39 && *c >= 0x30 {
                num1 = *c as i32- 0x30 as i32;
                break;
            }
        }
        if  i+2 < len && ONE == &bytes[i..=i+2] {
            num1 = 1;
            break;
        }
        else if i+2 < len &&TWO == &bytes[i..=i+2] {
            num1 = 2;
            break;
        }
        else if i+4 < len &&THREE == &bytes[i..=i+4] {
            num1 = 3;
            break;
        }
        else if i+3 < len &&FOUR== &bytes[i..=i+3] {
            num1 = 4;
            break;
        }
        else if i+3 < len &&FIVE == &bytes[i..=i+3] {
            num1 = 5;
            break;
        }
        else if i+2 < len &&SIX == &bytes[i..=i+2] {
            num1 = 6;
            break;
        }
        else if i+4 < len &&SEVEN == &bytes[i..=i+4] {
            num1 = 7;
            break;
        }
        else if i+4 < len && EIGHT == &bytes[i..=i+4] {
            num1 = 8;
            break;
        }
        else if i+3 < len && NINE == &bytes[i..=i+3] {
            num1 = 9;
            break;
        }
    }
    
    let rev_bytes = safe_rev(bytes);
    ONE.reverse();
    TWO.reverse();
    THREE.reverse();
    FOUR.reverse();
    FIVE.reverse();
    SIX.reverse();
    SEVEN.reverse();
    EIGHT.reverse();
    NINE.reverse();
    for i in 0..len {
        if let Some(c) = rev_bytes.get(i) {
            if *c <= 0x39 && *c >= 0x30 {
                num2 = *c as i32- 0x30 as i32;
                break;
            }
        }
        if i+2 < len && ONE == &rev_bytes[i..=i+2] {
            num2 = 1;
            break;
        }
        else if i+2< len && TWO == &rev_bytes[i..=i+2] {
            num2 = 2;
            break;
        }
        else if i+4< len &&THREE == &rev_bytes[i..=i+4] {
            num2 = 3;
            break;
        }
        else if i+3< len && FOUR== &rev_bytes[i..=i+3] {
            num2 = 4;
            break;
        }
        else if i+3< len && FIVE == &rev_bytes[i..=i+3] {
            num2 = 5;
            break;
        }
        else if i+2< len && SIX == &rev_bytes[i..=i+2] {
            num2 = 6;
            break;
        }
        else if i+4 < len && SEVEN == &rev_bytes[i..=i+4] {
            num2 = 7;
            break;
        }
        else if i+5 < len &&EIGHT == &rev_bytes[i..=i+4] {
            num2 = 8;
            break;
        }
        else if i+3 < len && NINE == &rev_bytes[i..=i+3] {
            num2 = 9;
            break;
        }
    }
    print!("{num1}{num2}\n");

    return num1 * 10 + num2;
}

