use std::{io::{Lines, BufRead, BufReader}, collections::HashMap, iter::repeat, thread, sync::Arc};
use anyhow::{Context,Error};
use num_integer;
// use num_bigint::BigUint;

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

fn followmap(directions:&String, maps:&HashMap<String,(String,String)>) -> u32 {
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
    return steps;
}

// use the lcm of the steps of each leg to figure out when they all line up
fn followmap_ghost_lcm(directions:Arc<String>, maps:Arc<HashMap<String,(String,String)>>) -> u64 {
    let locations:Vec<String> = { maps.iter().filter_map(
        |(k,_v)| {
            if k.ends_with("A") { Some(k.clone()) } else { None }
    }).collect()};
    if DEBUG { eprintln!("starting {locations:?}") };

    //let mut path_steps = Vec::new();
    let mut handles = Vec::new();

    for start in locations {
        let arc_directions = directions.clone();
        let arc_maps = maps.clone();
        let handle = thread::spawn(move ||{
            // thread::yield_now();
            let mut location = &start;
            let mut steps:u32 = 0;
            for step in repeat(arc_directions.chars()).flatten() {
                let fork = arc_maps.get(location).unwrap();
                location = match step {
                    'L' => &fork.0,
                    'R' => &fork.1,
                    _ => panic!("unknown step"),
                };
                steps += 1;
                // if DEBUG { eprintln!("{steps}: {step} => {location}") };
                if location.ends_with('Z') { 
                    if DEBUG { eprintln!("ending: {location}") };
                    break; 
                }
            }
            return steps;
        });

        handles.push(handle);
    }

    let path_steps:Vec<u32> = handles.into_iter().map(|h|h.join().unwrap()).collect();
    if DEBUG { eprintln!("path_steps: {path_steps:?}") };

    // in case of 64bit overflow.
    // let all_steps = path_steps.iter().fold(BigUint::from(1u32),|acc,nxt| num_integer::lcm(acc,BigUint::from(*nxt)));

    let all_steps = path_steps.iter().fold(1u64,|acc,nxt| num_integer::lcm(acc,*nxt as u64));
    if DEBUG { eprintln!("all_steps: {all_steps} [{} bits]", 64-all_steps.leading_zeros()) };
    return all_steps;
}

// in part 2, ghosts follow all paths simultaneously
fn followmap_ghost(directions:&String, maps:&HashMap<String,(String,String)>) -> u32 {
    let mut steps = 0;
    let mut locations:Vec<&String> = maps.iter().filter_map(
        |(k,_v)| {
            if k.ends_with("A") { Some(k) } else { None }
    }).collect();
    if DEBUG { eprintln!("starting {locations:?}") };
    for step in repeat(directions.chars()).flatten() {
        let mut new_locations = Vec::new();
        for location in locations {
            let fork = maps.get(location).unwrap();
            let new_location = match step {
                'L' => &fork.0,
                'R' => &fork.1,
                _ => panic!("unknown step"),
            };
            new_locations.push(new_location);
        }
        locations = new_locations;
        steps += 1;
        if DEBUG { eprintln!("{steps}: {step} => {locations:?}") };
        if locations.iter().all(|loc|loc.ends_with('Z')) { 
            break; 
        }
    }
    return steps;
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

    // PART TWO
    eprintln!("PART TWO");

    // follow the map steps
    let steps = followmap_ghost_lcm(Arc::new(directions), Arc::new(maps));

    // output the steps required
    println!("{steps}");

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

#[test]
fn part2_example() -> Result<(),Error> {
    let testinput = 
r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    go(&mut testinput.as_bytes())
}