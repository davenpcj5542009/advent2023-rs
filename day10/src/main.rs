use std::{io::{BufRead, BufReader}, path, fmt::{Display, Write}};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

const START:char = 'S';
const NE: char = 'L';
const NS: char = '|';
const NW: char = 'J';
const EW: char = '-';
const SW: char = '7';
const SE: char = 'F';

fn check_adjacent(directions: char, x: usize, y: usize, grid: &Vec<Vec<char>>) -> Option<i32> {
    unimplemented!();
}

fn check_connection(x:isize, y:isize, newx:isize, newy:isize, grid:&Vec<Vec<char>>) -> bool {
    if newx < 0 || newy < 0 || newx >= grid.len() as isize || newy >= grid[0].len() as isize {
        if DEBUG { eprintln!("CHECK [{newx},{newy}] out of bounds") };
        return false;
    }
    match (newx-x,newy-y,grid[newx as usize][newy as usize]) {
        (0,1, NW | EW | SW) => true,
        (0,-1, NE | EW | SE ) => true,
        (-1,0, SW | NS | SE ) => true,
        (1,0,NW | NS | NE ) => true,
        _ => false,
    }
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
    if grid[x as usize][y as usize] == '.' {
        return;
    }

    dist[x as usize][y as usize] = pathlen;

    let direction = grid[x as usize][y as usize];
    // go up first
    match direction {
        NE | NS | NW => {
            if DEBUG { eprintln!("{} NORTH: [{x},{}]", direction, y-1) };
            compute_follow_path(pathlen+1, grid, x-1, y, dist);
        }, _ => (),
    }

    match direction {
        SE | NS | SW => {
            if DEBUG { eprintln!("{} SOUTH: [{x},{}]", direction, y+1) };
            compute_follow_path(pathlen+1, grid, x+1, y, dist);
        }, _ => (),
    }

    // try left or right
    match direction {
        EW | SW | NW => {
            if DEBUG { eprintln!("{} WEST: [{},{y}]", direction, x-1) };
            compute_follow_path(pathlen+1, grid, x, y-1, dist);
        }, _ => (),
    }

    match direction {
        EW | SE | NE => {
            if DEBUG { eprintln!("{} EAST: [{},{y}]", direction, x+1) };
            compute_follow_path(pathlen+1, grid, x, y+1, dist);
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
    let (x,y) = (x as isize,y as isize);
    // figure out which way start goes
    if check_connection(x, y, x + 1, y , &grid) {
        compute_follow_path(1, &grid, x + 1, y, &mut dist);
    }
    if check_connection(x, y, x - 1, y, &grid) {
        compute_follow_path(1, &grid, x - 1, y, &mut dist);
    }
    if check_connection(x, y, x, y + 1, &grid) {
        compute_follow_path(1, &grid, x, y + 1, &mut dist);
    }
    if check_connection(x, y, x - 1, y - 1, &grid) {
        compute_follow_path(1, &grid, x, y - 1, &mut dist);
    }

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

impl<'a> std::fmt::Debug for Grid<'a, i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for v in self.0.iter() {
            f.write_char('[')?;
            for i in v.iter() {
                std::fmt::Debug::fmt(i, f)?;
                f.write_str(", ")?;
            }
            f.write_str("],\n")?;
        }
        Ok(())
    }
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

    let check_dist = compute_distances(&grid);

    eprintln!("{:?}", Grid::from(&grid));

    let distances = vec![
        vec![-1, -1, 4, 5, -1, ],
        vec![-1, 2, 3, 6, -1, ],
        vec![0, 1, -1, 7, 8, ],
        vec![1, 4, 5, 6, 7, ],
        vec![2, 3, -1, -1, -1, ],
    ];
    eprintln!("{:?}", Grid::from(&check_dist));

    assert_eq!(Grid::from(&check_dist), Grid::from(&distances));
}

#[test]
fn testinput2() {
    let testinput = 
r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    let grid = testinput.split_ascii_whitespace().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let check_dist = compute_distances(&grid);

    eprintln!("{:?}", Grid::from(&grid));

    let distances = vec![
        vec![-1, -1, -1, -1, -1, ],
        vec![-1, 0, 1, 2, -1, ],
        vec![-1, 1, -1, 3, -1, ],
        vec![-1, 2, 3, 4, -1, ],
        vec![-1, -1, -1, -1, -1, ],
    ];
    eprintln!("{:?}", Grid::from(&check_dist));

    assert_eq!(Grid::from(&check_dist), Grid::from(&distances));
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