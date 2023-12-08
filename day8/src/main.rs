use std::{io::{Lines, BufRead, BufReader}, collections::HashMap, iter::repeat};

use anyhow::{Context,Error};

const DEBUG:bool = cfg!(debug_assertions);


fn str_to_vec(somestr: &str, skip:usize) -> Vec<u32> {
    somestr.split_ascii_whitespace().skip(skip).map(|s| s.parse().expect("string to be sequence of u32")).collect()
}

fn loadmap(map:&mut HashMap<String,(String,String)>, line: &String) {

    let (key, paths) = line.split_once(" = ").unwrap();
    let (left,right) = paths.split_once(", ").unwrap();
    let left = left.trim_start_matches('(');
    let right = right.trim_end_matches(')');
    map.insert(key.to_owned(), (left.to_owned(), right.to_owned()));
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // map navigation
    // puzzle input, line of directions, lines of path forks
    let mut lines = BufReader::new(input).lines();

    let directions = lines.next().unwrap().unwrap();

    if DEBUG { eprintln!("directions: {:?}", &directions) };

    let mut maps = HashMap::new();

    if ! lines.next().expect("some line").expect("unexpected io error").is_empty() {
        panic!("next line wasn't empty");
    }

    while let Some(Ok(line)) = lines.next() {
        loadmap(&mut maps, &line);
    }

    if DEBUG { eprintln!("pathmap: {:?}", &maps) };

    // follow the map steps
    let mut steps = 0;
    let mut location = "AAA";
    for step in repeat(directions.chars()).flatten() {
        let fork = maps.get(location).unwrap();
        location = match step {
            'L' => fork.0.as_str(),
            'R' => fork.1.as_str(),
            _ => panic!("unknown step"),
        };
        steps += 1;
        if DEBUG { eprintln!("{steps}: {step} => {location}") };
        if location == "ZZZ" { 
            break; 
        }
    }

    // output the steps required
    println!("{steps}");

    // PART TWO
    // eprintln!("PART TWO");

    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    go(&mut testinput.as_bytes())
}

#[test]
fn example2() -> Result<(),Error> {
    let testinput = 
r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let mut rmap = HashMap::new();
    loadmap(&mut rmap, &"AAA = (BBB, CCC)".to_owned());

    assert_eq!(rmap.get("AAA"), Some(&("BBB".to_owned(), "CCC".to_owned())));
}

#[test]
fn test2() {
    let res:String = repeat("LR".chars()).flatten().take(20).collect();
    eprintln!("{res}");
    assert_eq!(res, String::from_iter(repeat("LR").take(10)));
    assert_eq!(res.len(), 20);
}