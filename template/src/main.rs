use std::io::{BufRead, BufReader};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

fn loadmap<T:BufRead>(lines: &mut Lines<T>) -> Result<(String, String, RangeInclusiveMap<u32, RangeInclusive<u32>>),Error> {
    // item-to-item map:
    // 50 98 2    // start output range, start input range, range length

    let mapname_str = lines.next().context("looking for item-to-item-map name string")?.unwrap();
    let (fromname, toname) = mapname_str.split_whitespace().next().with_context(||format!("missing space in map name '{}'", mapname_str))?.split_once("-to-").expect("missing separating phrase -to- in item-to-item map");

    let mut map = RangeInclusiveMap::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() { break; }

        let inputs = str_to_vec(&line,0);
        if DEBUG { eprintln!("mapinput '{line}'->{:?}", &inputs) }
        let t = inputs[0]..=inputs[0]+(inputs[2]-1);
        let f = inputs[1]..=inputs[1]+(inputs[2]-1);
        map.insert(f, t);
    }
    if DEBUG { eprintln!("item map: '{fromname}' to '{toname}'\n{:?}", &map) };
    Ok((fromname.to_owned(), toname.to_owned(), map))
}

fn str_to_vec(somestr: &str, skip:usize) -> Vec<u32> {
    somestr.split_ascii_whitespace().skip(skip).map(|s| s.parse().expect("string to be sequence of u32")).collect()
}

// take input range, return range of input which is left of target, if any, and range of input which is right of target, if any
fn trim_range(input: &RangeInclusive<u32>, target: &RangeInclusive<u32>) -> (Option<RangeInclusive<u32>>, Option<RangeInclusive<u32>>) {
    let mut left = None;
    let right ;
    if input.start() < target.start() {
        left = Some(
            if input.end() < target.start() {
                // non overlapping
                input.clone()
            } else {
                RangeInclusive::new(*input.start(), *target.start()-1)
            }
        );
    } 
    
    if input.start() <= target.end() {
        right = 
            if input.end() <= target.end() {
                // fully overlapping
                None
            } else {
                Some(RangeInclusive::new(*target.end()+1, *input.end()))
            }
        ;
    } else {
        // non overlapping
        right = Some(input.clone());
    }

    if DEBUG { eprint!(" => Trim({left:?},{right:?})")}
    
    (left, right)
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // NAME
    // puzzle input
    let mut lines = BufReader::new(input).lines();

    let some_str = lines.next().unwrap().unwrap();
    // "vals: 79 14 55 13"
    let somelist = str_to_vec(&some_str,1);

    if DEBUG { eprintln!("somelist: {:?}", &somelist) };

    let mut maps = HashMap::new();

    if ! lines.next().expect("some line").expect("unexpected io error").is_empty() {
        panic!("next line wasn't empty");
    }

    // item-to-item map:
    // 50 98 2    // start output range, start input range, range length

    while let Ok(map) = loadmap(&mut lines) {
        maps.insert((map.0, map.1), map.2);
    }

    // compute the result
    let result = 0;

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
r"vals: 79 14 55 13
";
    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = "vals: 79 14 55 13\n";
    assert_eq!(str_to_vec(&testinput,1), [79,14,55,13]);
}

#[test]
fn test2() {
}
