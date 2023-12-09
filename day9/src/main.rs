use std::{io::{BufRead, BufReader}, mem::discriminant};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

fn str_to_vec(somestr: &str, skip:usize) -> Vec<i32> {
    somestr.split_ascii_whitespace().skip(skip).map(|s| s.parse().expect("string to be sequence of i32")).collect()
}

fn get_discriminant_next(sensors:&[i32]) -> i32 {
    let next:Vec<i32> = sensors.windows(2).map(|sns| sns[1] - sns[0]).collect();
    if DEBUG { eprintln!(" {sensors:?} => {next:?}") }
    if next.iter().any(|&sn| sn != 0) {
        let r = sensors[sensors.len()-1] + get_discriminant_next(&next);
        return r;
    } else {
        if DEBUG { eprintln!(" {}", sensors[0]) }        
        return sensors[0];
    }
}

fn get_discriminant_prev(sensors:&[i32]) -> i32 {
    let next:Vec<i32> = sensors.windows(2).map(|sns| sns[1] - sns[0]).collect();
    if DEBUG { eprintln!(" {sensors:?} => {next:?}") }
    if next.iter().any(|&sn| sn != 0) {
        let r = sensors[0] - get_discriminant_prev(&next);
        return r;
    } else {
        if DEBUG { eprintln!(" {}", sensors[0]) }        
        return sensors[0];
    }
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // Mirage maintenance
    // puzzle input, line of values in a history
    let mut lines = BufReader::new(input).lines();

    // "0 3 6 9 12 15"
    let mut result = 0;
    while let Some(Ok(line)) = lines.next() {
        let sensors = str_to_vec(&line,0);
        // compute the result
        let order_next = get_discriminant_prev(&sensors);
        if DEBUG { eprintln!("{sensors:?}, {order_next}") };
        result += order_next;
    }

    // output the result
    println!("{result}");

    // PART TWO. 
    // eprintln!("PART TWO");
    // println!("{min_loc}");

    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = "vals: 79 14 55 13\n";
    assert_eq!(str_to_vec(&testinput,1), [79,14,55,13]);
}

#[test]
fn test2() {
    let v = [1,2,3,4,5];
    assert_eq!(get_discriminant_next(&v), 6);
}

#[test]
fn test3() {
    let v = [1,2,3,4,5];
    assert_eq!(get_discriminant_prev(&v), 0);
}
