use std::io::{BufRead, BufReader};
use anyhow::Error;

const DEBUG:bool = cfg!(debug_assertions);

#[cfg(test)]
fn str_to_vec(somestr: &str, skip:usize) -> Vec<u32> {
    somestr.split_ascii_whitespace().skip(skip).map(|s| u32::from_str_radix(s, 10).expect("string to be sequence of u32")).collect()
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // trebuchet calibration
    // 1abc2
    // first and last number combine to form a two digit number
    
    let mut lines = BufReader::new(input).lines();

    let mut calib_sum = 0;

    while let Some(Ok(calib_str)) = lines.next() {
        let first = calib_str.find(|c:char|c.is_ascii_digit()).unwrap();
        let last = calib_str.rfind(|c:char|c.is_ascii_digit()).unwrap();

        let calib:u32 = [first,last].iter().map(|idx|calib_str.chars().nth(*idx).unwrap()).collect::<String>().parse().expect("a number");
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
fn testinput1() {
    let testinput = "Time:      7  15  30\n";
    assert_eq!(str_to_vec(&testinput,1), [7,15,30]);
}

#[test]
fn test() {
}

