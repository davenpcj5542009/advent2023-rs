use std::io::{Lines, BufRead, BufReader};

use anyhow::{Context,Error};

const DEBUG:bool = cfg!(debug_assertions);

fn str_to_vec(somestr: &str, skip:usize) -> Vec<u32> {
    somestr.split_ascii_whitespace().skip(skip).map(|s| u32::from_str_radix(s, 10).expect("string to be sequence of u32")).collect()
}

fn str_to_joined(somestr: &str, skip:usize) -> u64 {
    let s:String = somestr.split_ascii_whitespace().skip(skip).collect();
    if DEBUG { eprintln!("'{}' => {}", somestr, &s) };   
    s.parse().expect("u64 string")
}

const ACCEL:u32 = 1; // 1 ms-per-ms

fn distance(hold_time: u32, race_time: u32) -> u64 {
    if hold_time >= race_time {
        return 0;
    }

    return (race_time-hold_time) as u64 *(hold_time as u64);
}

fn ways_to_win(racetime: u32, record: u64) -> u32 {
    let mut ways_to_win = 0;
    for t in 1..racetime {
        if distance(t, racetime) > record {
            ways_to_win += 1;
        }
    }
    if DEBUG { eprintln!("race wins: {:?}", &ways_to_win) };
    return ways_to_win;
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // boat racing
    //Time:      7  15  30
    //Distance:  9  40 200

    // first, second, third races
    // whole time for race, then record distance for races
    
    let mut lines = BufReader::new(input).lines();

    let times_str = lines.next().unwrap().unwrap();
    let dist_str = lines.next().unwrap().unwrap();

    let times = str_to_vec(&times_str, 1);
    let records = str_to_vec(&dist_str, 1);

    if DEBUG { eprintln!("times: {:?}", &times_str) };

    
    let mut margin_error = 1;
    for race in 0..times.len() {
        margin_error *= ways_to_win(times[race], records[race] as u64);
    }
    
    if DEBUG { eprintln!("margin_error: {:?}", &margin_error) };

    eprintln!("PART TWO");

    let times = str_to_joined(&times_str, 1) as u32;
    let record = str_to_joined(&dist_str, 1);

    let mut margin_error = ways_to_win(times, record);
    
    if DEBUG { eprintln!("margin_error: {:?}", &margin_error) };

    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"Time:      7  15   30
Distance:  9  40  200";

    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = "Time:      7  15  30\n";
    assert_eq!(str_to_vec(&testinput,1), [7,15,30]);
}

#[test]
fn test() {
    assert_eq!(distance(1, 7), 6);
    assert_eq!(distance(2, 7), 10);
    assert_eq!(distance(3, 7), 12);
    assert_eq!(distance(4, 7), 12);
    assert_eq!(distance(5, 7), 10);
    assert_eq!(distance(6, 7), 6);
}

