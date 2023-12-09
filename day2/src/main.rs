use std::io::{BufRead, BufReader};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

#[derive(Debug,PartialEq)]
struct RGB(u32,u32,u32);

impl From<&str> for RGB {
    fn from(s: &str) -> Self {
        let mut retv = RGB(0,0,0);

        for item in s.split(", ") {
            match item.split_once(' ').unwrap() {
                (r,"red") => retv.0 = r.parse().unwrap(),
                (g,"green") => retv.1 = g.parse().unwrap(),
                (b,"blue") => retv.2 = b.parse().unwrap(),
                _ => panic!("unknown item {item:?}"),
            }
        }

        if DEBUG { eprintln!("'{s}'=>{retv:?}")}
        return retv;
    }
}

fn str_to_vec(game_str: &str) -> Vec<RGB> {
    
    return game_str.split("; ").map(RGB::from).collect();
}

const GAME_LIMIT:RGB = RGB(12,13,14);
fn check_game(game: &Vec<RGB>) -> bool {
    let check = !game.iter().any(|val| val.0>GAME_LIMIT.0 || val.1>GAME_LIMIT.1 || val.2>GAME_LIMIT.2);
    if DEBUG { eprintln!("'{game:?}'=>{check}")}
    return check;
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // cube conundrum
    // puzzle input
    let mut lines = BufReader::new(input).lines();

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green   // game: reveal1; reveal2; reveal3
    let mut games = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let (id_str,game_str) = line.split_once(": ").unwrap();
        games.push((id_str.split(' ').nth(1).unwrap().parse::<u32>().unwrap(),str_to_vec(&game_str)));
    }

    // compute the result
    let mut game_count = 0;
    for game in games.iter() {
        if check_game(&game.1) {
            game_count += game.0;
        }
    }

    // output the result
    println!("{game_count}");

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
r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    assert_eq!(str_to_vec(&testinput).as_slice(), [RGB(4,0,3),RGB(1,2,6),RGB(0,2,0)]);
}

#[test]
fn test2() {
}
