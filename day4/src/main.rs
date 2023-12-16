use std::io::{BufRead, BufReader};
use std::collections::HashSet;

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

fn str_to_vec(somestr: &str, skip:usize) -> Vec<u32> {
    somestr.split_ascii_whitespace().skip(skip).map(|s| s.parse().expect("string to be sequence of u32")).collect()
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // Elf Scratch-offs
    // puzzle input  "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    let mut lines = BufReader::new(input).lines();

    let mut points = 0;
    while let Some(Ok(some_str)) = lines.next() {
        let (cardstr, numstr) = some_str.split_once(" | ").unwrap();
        // "vals: 79 14 55 13"
        let card = str_to_vec(cardstr,2);
        let nums = str_to_vec(numstr, 0);

        let matches = nums.iter().filter(|&n|card.contains(n)).count();
        if matches > 0 {
            points += 1<<(matches-1);
        }
    }

    // output the result
    println!("{points}");

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
r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    go(&mut testinput.as_bytes())
}

