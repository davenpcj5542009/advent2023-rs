use std::io::{BufRead, BufReader};
use anyhow::Error;

const DEBUG:bool = cfg!(debug_assertions);

#[cfg(test)]
fn str_to_vec(somestr: &str, skip:usize) -> Vec<u32> {
    somestr.split_ascii_whitespace().skip(skip).map(|s| u32::from_str_radix(s, 10).expect("string to be sequence of u32")).collect()
}

fn get_calibration(calib_str:&String) -> u32 {
    let first = calib_str.find(|c:char|c.is_ascii_digit()).unwrap();
    let last = calib_str.rfind(|c:char|c.is_ascii_digit()).unwrap();

    let calib:u32 = [first,last].iter().map(|idx|calib_str.chars().nth(*idx).unwrap()).collect::<String>().parse().expect("a number");
    if DEBUG { eprintln!("{calib_str} => {calib}") };

    return calib;
}

const WORDVALUES:[&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn word_to_value(value_str:&str) -> u32 {
    if let Ok(val) = value_str.parse() {
        return val;
    }
    WORDVALUES.iter().position(|&v| v == value_str).unwrap() as u32
}

fn get_calibration_words(calib_str:&String) -> u32 {
    // in part 2, spelled words can form digits
    let mut first = 0;

    'strloop:
    for pos in 0..calib_str.len() {
        for word in 0..WORDVALUES.len() {
            if calib_str.chars().nth(pos).unwrap().is_ascii_digit() {
                first = calib_str.chars().nth(pos).unwrap().to_digit(10).unwrap();
                break 'strloop;
            }
            if calib_str.get(pos..).unwrap().starts_with(&WORDVALUES[word]) {
                first = word as u32;
                break 'strloop;
            }
        }
    }

    let mut last = 0;
    'strloop:
    for pos in (0..calib_str.len()).rev() {
        for word in 0..WORDVALUES.len() {
            if calib_str.chars().nth(pos).unwrap().is_ascii_digit() {
                last = calib_str.chars().nth(pos).unwrap().to_digit(10).unwrap();
                break 'strloop;
            }
            if calib_str.get(pos..).unwrap().starts_with(&WORDVALUES[word]) {
                last = word as u32;
                break 'strloop;
            }
        }
    }

    return first*10+last;
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // trebuchet calibration
    // 1abc2
    // first and last number combine to form a two digit number
    
    let mut lines = BufReader::new(input).lines();

    let mut calib_sum = 0;
    println!("PART TWO");

    while let Some(Ok(calib_str)) = lines.next() {
        let calib:u32 = get_calibration_words(&calib_str);
        if DEBUG { eprintln!("{calib_str} => {calib}") };
        calib_sum += calib;
    }
    
    println!("{calib_sum}");

    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    go(&mut testinput.as_bytes())
}

#[test]
fn example2() -> Result<(),Error> {
    let testinput = 
r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = "Time:      7  15  30\n";
    assert_eq!(str_to_vec(&testinput,1), [7,15,30]);
}

#[test]
fn test() {
}

