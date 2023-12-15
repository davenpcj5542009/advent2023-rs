use std::{io::{BufRead, BufReader}, path, fmt::{Display, Write}, ops::{Index, Range}};

use anyhow::{Error};
use regex::{Regex, RegexBuilder};

const DEBUG:bool = cfg!(debug_assertions);

fn str_to_vec(somestr: &str) -> (String,Vec<i32>) {
    let mut splitter = somestr.split_ascii_whitespace();
    let left = splitter.next().unwrap();
    let right = splitter.next().unwrap();

    let vec = right.split(',').map(|s| s.parse().expect("string to be sequence of i32")).collect();
    return (String::from(left), vec);
}

// may need our own FSM to match these
// do a recursive backtracking match
pub fn match_groups(pos:usize, remaining: &str, groups: &[i32], matches:&mut Vec<Range<usize>>) {
    let mut i = 0;
    if DEBUG { eprintln!("in [{pos}..]'{remaining}', matching {groups:?} ") };

    // consume the empty space, if any
    while i < remaining.len() && remaining.as_bytes()[i] == b'.' {
        i += 1;
    }

    if i == remaining.len() && groups.len() == 0 {
        if DEBUG { eprintln!("matched ({pos}..{i}) ") };
        matches.push(pos..i);
        return;
    }

    // consume the first "groups", if possible
    let mut n = 0;
    while i < remaining.len() && n < groups[0] && (remaining.as_bytes()[i] == b'#' || remaining.as_bytes()[i] == b'?') {
        n += 1;
        i += 1;
    }

    if n < groups[0] || (i < remaining.len() && remaining.as_bytes()[i] == b'#') {
        if DEBUG { eprintln!("no match segment {} at {i}", groups[0]) };
        return;
    }

    if i == remaining.len() && n == groups[0] && groups.len() == 1{
        if DEBUG { eprintln!("matched ({pos}..{i}) ") };
        matches.push(pos..i);
        return;
    }

    // must consume at least one separator
    i += 1;

    // try to match the rest
    match_groups(pos, &remaining[i..], &groups[1..], matches);
}

fn build_regex(groups: &Vec<i32>) -> Regex {
    let re_str:String = groups.iter().map(|i|format!("[#\\?]{{{i}}}")).collect::<Vec<String>>().join("[\\.\\?]+");

    if DEBUG { eprintln!("{groups:?} => {re_str}") };
    //return Regex::new(&re_str).configure(Config::new().match_kind(MatchKind::All));
    return Regex::new(&re_str).unwrap();
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // spring repair
    // puzzle input, list of springs, list of spring groups.
    let mut lines = BufReader::new(input).lines();
    let mut result = 0;

    while let Some(Ok(line)) = lines.next() {
        let (springs,groups) = str_to_vec(&line);

        // create regex to match possible arrangements
        let re = build_regex(&groups);

        // count matches
        let mut matches = Vec::new();
        match_groups(0, &springs, &groups, &mut matches);
        //let matches:Vec<_> = re.find_iter(&springs).map(|m|m.as_str()).collect();
        if DEBUG { eprintln!("matches: {matches:?}") };
        let steps = matches.len();
        if DEBUG { eprintln!("-> {steps:?}") };
        result += steps;
    }


    // PART TWO
    //eprintln!("PART TWO");

    // output the result
    println!("{result}");

    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[derive(PartialEq)]
struct Grid<'a,T:PartialEq> (&'a Vec<Vec<T>>);

impl<'a, T:PartialEq> From<&'a Vec<Vec<T>>> for Grid<'a,T> {
    fn from(g: &'a Vec<Vec<T>>) -> Self {
        Self(g)
    }
}

impl<'a> std::fmt::Debug for Grid<'a, char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for v in self.0.iter() {
            f.write_char('[')?;
            for c in v.iter() {
                f.write_char(*c)?;
            }
            f.write_str("],\n")?;
        }
        Ok(())
    }
}

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";  

    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = 
r"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

    let retv:Vec<_> = testinput.lines().map(str_to_vec).collect();

    assert_eq!(retv[0], (String::from("#.#.###"), vec![1,1,3]));
}

