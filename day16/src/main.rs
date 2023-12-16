use std::{io::{BufRead, BufReader}, path, fmt::{Display, Write}, ops::Index};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);
// X is rows, Y is columns, because grid[x][y].
// recursively follow the path to next passed postition
fn compute_follow_path(direction:char, grid:&Vec<Vec<char>>, x:isize, y:isize, energy:&mut Vec<Vec<i32>>) {

    if DEBUG { eprintln!("Going {direction} to [{x},{y}]") };

    // bounds check
    if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
        if DEBUG { eprintln!("[{x},{y}] out of bounds") };
        return;
    }

    let p = "NSEW".chars().position(|c|c==direction).unwrap();

    // already been here (potentially 2 directions, either NS or EW)
    if energy[x as usize][y as usize] & 1<<p != 0 {
        if DEBUG { eprintln!("[{x},{y}] already traversed") };
        return;
    }

    // light the square
    energy[x as usize][y as usize] |= 1<<p;

    let redirector = grid[x as usize][y as usize];
    // go up first
    match (direction,redirector) {
        // continue in the same direction
        ('E','.' | '-') | ('N', '/') | ('S', '\\') => compute_follow_path('E', grid, x, y+1, energy),
        ('W','.' | '-') | ('S', '/') | ('N', '\\') => compute_follow_path('W', grid, x, y-1, energy),
        ('N','.' | '|') | ('E', '/') | ('W', '\\') => compute_follow_path('N', grid, x-1, y, energy),
        ('S','.' | '|') | ('W', '/') | ('E', '\\') => compute_follow_path('S', grid, x+1, y, energy),
        ('E'|'W', '|') => {
            compute_follow_path('N', grid, x - 1, y, energy);
            compute_follow_path('S', grid, x + 1, y, energy);
        },
        ('N'|'S', '-') => {
            compute_follow_path('E', grid, x, y + 1, energy);
            compute_follow_path('W', grid, x, y - 1, energy);
        },
        _ => panic!("bad direction '{direction}', redirection '{redirector}'"),
    }
}

fn load_grid(input:&mut dyn BufRead) -> Result<Vec<Vec<char>>,Error> {
    let mut lines = input.lines();
    let mut grid = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            return Ok(grid);
        }
        grid.push(line.chars().collect::<Vec<_>>());
    }
    anyhow::ensure!(!grid.is_empty(), "No more");
    return Ok(grid);
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // beam reflecting heater
    // puzzle input, grid of directing mirrors and splitters
    let mut grid = load_grid(input).unwrap();

    if DEBUG { eprintln!("grid: {:?}", Grid::from(&grid)) };

    // get the result grid ready
    let mut energy = grid.iter().map(|r|r.iter().map(|sq|0i32).collect::<Vec<_>>()).collect::<Vec<_>>();

    // walk the grid, top left, heading right
    compute_follow_path('E', &grid,0, 0, &mut energy);

    if DEBUG { eprintln!("energies: {:X?}", Grid::from(&energy)) };

    let squares = energy.iter().flatten().filter(|&v|*v!=0).count();
    // output the energy count
    println!("{squares}");

    // PART TWO
    eprintln!("PART TWO");

    let mut max_squares = squares;
    // find the entry point and direction which maximizes the energy.
    for left in 1..grid.len() {
        energy.iter_mut().flatten().for_each(|v|*v=0);
        compute_follow_path('E', &grid, left as isize, 0, &mut energy);
        let squares = energy.iter().flatten().filter(|&v|*v!=0).count();
        if squares > max_squares {
            max_squares = squares;
        }
    }

    for right in 0..grid.len() {
        energy.iter_mut().flatten().for_each(|v|*v=0);
        compute_follow_path('W', &grid, right as isize, grid[0].len()as isize-1, &mut energy);
        let squares = energy.iter().flatten().filter(|&v|*v!=0).count();
        if squares > max_squares {
            max_squares = squares;
        }
    }

    for up in 0..grid[0].len() {
        energy.iter_mut().flatten().for_each(|v|*v=0);
        compute_follow_path('N', &grid, grid.len() as isize-1, up as isize, &mut energy);
        let squares = energy.iter().flatten().filter(|&v|*v!=0).count();
        if squares > max_squares {
            max_squares = squares;
        }
    }

    for down in 0..grid[0].len() {
        energy.iter_mut().flatten().for_each(|v|*v=0);
        compute_follow_path('S', &grid, 0, down as isize, &mut energy);
        let squares = energy.iter().flatten().filter(|&v|*v!=0).count();
        if squares > max_squares {
            max_squares = squares;
        }
    }

    println!("{max_squares}");

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

impl<'a,T:std::fmt::Debug + std::cmp::PartialEq> std::fmt::Debug for Grid<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for v in self.0.iter() {
            f.write_char('[')?;
            for c in v.iter() {
                c.fmt(f)?;
            }
            f.write_str("],\n")?;
        }
        Ok(())
    }
}


#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    go(&mut testinput.as_bytes())
}

