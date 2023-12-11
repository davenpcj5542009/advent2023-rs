use std::{io::{BufRead, BufReader}, path};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

const START:char = 'S';
const NE: char = 'L';
const NS: char = '|';
const NW: char = 'J';
const EW: char = '-';
const SW: char = '7';
const SE: char = 'F';

fn check_adjacent(directions: char, x: usize, y: usize, dist: &Vec<Vec<i32>>) -> Option<i32> {
    unimplemented!();
}

// recursively follow the path from current postition, whose value must be known
fn compute_follow_path(pathlen: i32, grid:&Vec<Vec<char>>, x:isize, y:isize, dist:&mut Vec<Vec<i32>>) {
    // bounds check
    if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
        if DEBUG { eprintln!("[{x},{y}] out of bounds") };
        return;
    }
    if dist[x as usize][y as usize] != -1 && dist[x as usize][y as usize] < pathlen {
        if DEBUG { eprintln!("dist [{x},{y}] already set {}", dist[x as usize][y as usize]) };
        return;
    }
    dist[x as usize][y as usize] = pathlen;

    let direction = grid[x as usize][y as usize];
    // go up first
    match direction {
        NE | NS | NW => {
            if DEBUG { eprintln!("{} NORTH: [{x},{}]", direction, y-1) };
            compute_follow_path(pathlen+1, grid, x, y-1, dist);
        }, _ => (),
    }

    match direction {
        SE | NS | SW => {
            if DEBUG { eprintln!("{} SOUTH: [{x},{}]", direction, y+1) };
            compute_follow_path(pathlen+1, grid, x, y+1, dist);
        }, _ => (),
    }

    // try left or right
    match direction {
        EW | SW | NW => {
            if DEBUG { eprintln!("{} WEST: [{},{y}]", direction, x-1) };
            compute_follow_path(pathlen+1, grid, x-1, y, dist);
        }, _ => (),
    }

    match direction {
        EW | SE | NE => {
            if DEBUG { eprintln!("{} EAST: [{},{y}]", direction, x+1) };
            compute_follow_path(pathlen+1, grid, x+1, y, dist);
        }, _ => (),
    }
}

fn compute_distances(grid:&Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut dist: Vec<Vec<i32>> = grid.iter().map(|row|row.iter().map(|_c|-1).collect()).collect();

    let mut x = 0;
    let mut y = 0;
    'search:
    while x < grid.len() {
        y = 0;
        while y < grid[x].len() {
            if grid[x][y] == START {
                if DEBUG { eprintln!("FOUND START: [{x},{y}]") };
                break 'search;
            }
            y += 1;
        }
        x += 1;
    }
    if DEBUG { eprintln!("START: [{x},{y}]") };
    dist[x][y] = 0;
    // figure out which way start goes

    compute_follow_path(0, grid, x as isize, y as isize, &mut dist);

    return dist;
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // pipe traversal
    // puzzle input, grid of pipe connections
    let mut lines = BufReader::new(input).lines();

    let grid = lines.map(|line|line.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    if DEBUG { eprintln!("grid: {:?}", &grid) };

    // compute the distances
    let dist = compute_distances(&grid);

    if DEBUG { eprintln!("dist: {:?}", &dist) };

    // find the farthest
    let steps = dist.iter().map(|row|row.iter()).flatten().max().unwrap();

    // PART TWO
    // eprintln!("PART TWO");

    // follow the map steps

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
r".....
.S-7.
.|.|.
.L-J.
.....";
    go(&mut testinput.as_bytes())
}

#[test]
fn example2() -> Result<(),Error> {
    let testinput = 
r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = 
r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    let grid = testinput.split_ascii_whitespace().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    eprintln!("{grid:?}");

    let check_dist = compute_distances(&grid);

    let distances:&[&[i32]] = &[
        &[-1, -1, 4, 5, -1, ],
        &[-1, 2, 3, 6, -1, ],
        &[0, 1, -1, 7, 8, ],
        &[1, 4, 5, 6, 7, ],
        &[2, 3, -1, -1, -1, ],
    ];

   assert_eq!(check_dist, distances);
}

#[test]
fn test2() {
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