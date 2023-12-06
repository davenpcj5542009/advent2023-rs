use std::collections::{HashMap, VecDeque};
use std::io::{Lines, BufRead, BufReader};
use std::ops::{Range, RangeInclusive};

use anyhow::{Context,Error};
use rangemap::RangeInclusiveMap;

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

fn convert_location(seed: u32, maps: &Vec<&RangeInclusiveMap<u32,RangeInclusive<u32>>>) -> u32 {
    let mut dest = seed;
    if DEBUG { eprint!("{seed}") };
    for &map in maps {
        dest = match map.get_key_value(&dest) {
            // 79, given 50..(50+48) => 52, should be 81
            Some((input,output)) => { 
                if DEBUG { eprint!("{{{dest}-{}+{}}}", input.start(), output.start())}
                dest - input.start() + output.start() 
            },
            None => dest,
        };
        if DEBUG { eprint!(" => {dest}") };
    }
    if DEBUG { eprintln!("; {seed} => {dest}") };
    return dest;
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // food projection problem
    // puzzle input, almanac listing seed, soil, fertilizer, etc "item-to-item-map:"s
    let mut lines = BufReader::new(input).lines();

    let seedlist_str = lines.next().unwrap().unwrap();
    // "seeds: 79 14 55 13"
    let seedlist = str_to_vec(&seedlist_str,1);

    if DEBUG { eprintln!("seedlist: {:?}", &seedlist) };

    let mut maps = HashMap::new();

    if ! lines.next().expect("some line").expect("unexpected io error").is_empty() {
        panic!("next line wasn't empty");
    }

    // item-to-item map:
    // 50 98 2    // start output range, start input range, range length

    while let Ok(map) = loadmap(&mut lines) {
        maps.insert((map.0, map.1), map.2);
    }

    // find location for each seed, following all the maps.
    // build a deque of seeds-to-*-to-location
    let mut deque = Vec::new();
    let mut current = "location";
    'mapsearch:
    loop {
        for entry in maps.keys() {
            let strentry = (entry.0.as_str(), entry.1.as_str());

            if strentry == ("seed", current) {  /* terminus */
                if DEBUG { eprint!("{} <= ", current); }
                deque.insert(0,maps.get(entry).unwrap());
                break 'mapsearch;
            }
            
            if strentry.1 == current { /* matching intermediate */ 
                if DEBUG { eprint!("{} <= ", current); }
                deque.insert(0,maps.get(&entry).unwrap());
                current = strentry.0;
                continue 'mapsearch;    // restart loop
            }
            /* non-matching */            
        }
        panic!("map list couldn't complete\nfor maps: {:?}", maps);
    }
    if DEBUG { eprintln!("seed"); }

    // output the minimum location found for any seed
    let min_loc = seedlist.iter().map(|seed|convert_location(*seed, &deque)).min().expect("empty seedlist");
    println!("{min_loc}");

    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = "seeds: 79 14 55 13\n";
    assert_eq!(str_to_vec(&testinput,1), [79,14,55,13]);
}

#[test]
fn testinput2() {
    let testinput = 
    r"seed-to-soil map:
    50 98 2
    52 50 48

    ";
    let mut rmap: RangeInclusiveMap<u32,RangeInclusive<u32>> = RangeInclusiveMap::new();
    rmap.insert(98..=100, 50..=52);
    rmap.insert(50..=98, 52..=100);
    assert_eq!(loadmap(&mut testinput.as_bytes().lines()).unwrap(), (String::from("seed"), String::from("soil"), rmap));
}

#[test]
fn testinput3() {
    let testinput = 
    r"seed-to-soil map:
    50 98 2
    52 50 48

    ";
    let rmap = loadmap(&mut testinput.as_bytes().lines()).unwrap();

    let mut v = Vec::new();
    v.push(&rmap.2);

    assert_eq!(convert_location(79, &v), 81);
}

#[test]
fn test3() {
    let mut rmap: RangeInclusiveMap<u32,RangeInclusive<u32>> = RangeInclusiveMap::new();
    rmap.insert(50..=52, 98..=100);
    rmap.insert(52..=100,50..=98);

    let mut v = Vec::new();
    v.push(&rmap);

    assert_eq!(convert_location(1, &v), 1);
    assert_eq!(convert_location(50, &v), 98);
    assert_eq!(convert_location(51, &v), 99);
    assert_eq!(convert_location(52, &v), 50);
    assert_eq!(convert_location(53, &v), 51);
    assert_eq!(convert_location(101, &v), 101);
}

#[test]
fn test4() {
    let mut v = Vec::new();

    let mut rmap: RangeInclusiveMap<u32,RangeInclusive<u32>> = RangeInclusiveMap::new();
    rmap.insert(50..=52, 76..=79);
    rmap.insert(52..=100,50..=98);
    v.push(&rmap);

    let mut rmap: RangeInclusiveMap<u32,RangeInclusive<u32>> = RangeInclusiveMap::new();
    rmap.insert(75..=77, 98..=100);
    rmap.insert(102..=150,50..=98);
    v.push(&rmap);

    assert_eq!(convert_location(1, &v), 1);
    assert_eq!(convert_location(50, &v), 99);
    assert_eq!(convert_location(51, &v), 77);
    assert_eq!(convert_location(52, &v), 50);
    assert_eq!(convert_location(53, &v), 51);
    assert_eq!(convert_location(101, &v), 101);
}

//let mut h = HashMap::new();
//h.insert((String::from("seed"), String::from("location")), rmap);
