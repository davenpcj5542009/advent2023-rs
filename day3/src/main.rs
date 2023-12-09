use std::{io::{BufRead, BufReader}, ops::Range};

use anyhow::{Error};
use regex::Regex;

const DEBUG:bool = cfg!(debug_assertions);

fn check_adjacent(ln: usize,rg:&Range<usize>,sym_ln:usize,sym_pos:usize) -> bool {
    let retv = (sym_ln == ln || sym_ln + 1 == ln || ln + 1 == sym_ln) && ( sym_pos + 1 >= rg.start && sym_pos <= rg.end);
    if DEBUG { eprintln!("[{ln},{rg:?}] adjacent to ({sym_ln},{sym_pos}) => {retv}") }
    return retv;
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // gondola lift gear ratios
    // puzzle input is numbers and symbols in grid
    let mut lines = BufReader::new(input).lines();

    let re = Regex::new("[0-9]+").unwrap();
    let mut sernums = Vec::new();
    let mut grid = Vec::new();
    let mut linenum = 0;
    // "617*......"
    while let Some(Ok(line)) = lines.next() {
        let match_nums = re.find_iter(&line);
        sernums.extend(match_nums.map(|m|(linenum, m.range(),m.as_str().parse::<u32>().unwrap())));
        grid.push(line);
        linenum += 1;
    }

    if DEBUG { eprintln!("{grid:?}") }

    if DEBUG { eprintln!("{sernums:?}") }

    // find all the symbols
    let mut symbols = Vec::new();
    for (linenum,line) in grid.iter().enumerate() {
        for (i,c) in line.char_indices() {
            if c.is_ascii_digit() || c == '.' {
                continue;
            }
            symbols.push((linenum,i as usize,c));
        }
    }

    // compute the result
    let mut result = 0;
    for &(ln,ref rg,sernum) in sernums.iter() {
        if DEBUG { eprintln!("SN {sernum}:") }
        for &(sym_ln,sym_pos,_sym) in symbols.iter() {            
            if check_adjacent(ln,&rg,sym_ln,sym_pos) {
                result += sernum;
                break; // we already found it, so no need to check more.
            }
        }
    }

    // output the result
    println!("{result}");

    // PART TWO. 
    eprintln!("PART TWO");
    let mut result = 0;
    for &(sym_ln,sym_pos,sym) in symbols.iter() {       
        if sym != '*' {
            continue;
        }
        if DEBUG { eprintln!("gear {sym_ln},{sym_pos}:") }

        let mut near_gear = sernums.iter().filter_map(|&(ln, ref rg, sernum)| {
            check_adjacent(ln,&rg,sym_ln,sym_pos).then(||sernum)
        });

        if DEBUG { eprintln!("near_gear: {near_gear:?}") }

        match (near_gear.next(),near_gear.next(),near_gear.next()) {
            (Some(first),Some(second),None) => {
                if DEBUG { eprintln!("ratio: {}", (first * second)) }
                result += first * second;
            },
            _ => if DEBUG { eprintln!("not a gear") },
        }
    }

    println!("{result}");
    return Ok(());
}

fn main() -> Result<(),Error> {
    go(&mut std::io::stdin().lock())
}

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    // let testinput = "vals: 79 14 55 13\n";
    // assert_eq!(str_to_vec(&testinput,1), [79,14,55,13]);
}

#[test]
fn test2() {
    // 0:..592#####
    // 1:.....#755#  // test should succeed for '#' areas
    // 2:...$.#####
    let rg = 4..7;
    for sym_pos in 3..=7 {
        assert_eq!(check_adjacent(1, &rg, 0, sym_pos), true);
        assert_eq!(check_adjacent(1, &rg, 1, sym_pos), true);
        assert_eq!(check_adjacent(1, &rg, 2, sym_pos), true);
    }

    // negative bound tests
    assert_eq!(check_adjacent(1,&rg, 3, 4), false);
    assert_eq!(check_adjacent(1,&rg, 1, 2), false);
    assert_eq!(check_adjacent(1,&rg, 1, 8), false);
    assert_eq!(check_adjacent(1,&rg, 0, 2), false);
}
