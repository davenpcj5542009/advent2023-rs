use std::{io::{BufRead, BufReader}, path, fmt::{Display, Write}, ops::Index};

use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

fn insert_row(row:usize, grid: &mut Vec<Vec<char>>) {
    grid.insert(row, grid[row].clone());
}

fn insert_col(col:usize, grid: &mut Vec<Vec<char>>) {
    let mut row = 0;
    while row < grid.len() {
        let element = grid[row][col].clone();
        grid[row].insert(col, element);
        row += 1;
    }
}

fn expand_universe(grid: &mut Vec<Vec<char>>) {    
    let mut row = 0;
    let mut col = 0;
    // if row empty, insert new row above
    while row < grid.len() {
        if grid[row].iter().all(|&c|c == '.') {
            if DEBUG { eprintln!("Expanding row {row}") };
            insert_row(row, grid);
            row += 1;
        }
        row += 1;
    }

    // if column empty, insert new column left
    while col < grid[0].len() {
        if grid.iter().map(|rv|rv[col]).all(|c|c=='.') {
            if DEBUG { eprintln!("Expanding col {col}") };
            insert_col(col, grid);
            col += 1;
        }
        col += 1;
    }
}

fn expand_old_universe(grid: &Vec<Vec<char>>) -> (Vec<usize>,Vec<usize>) {
    let mut expanded_rows = Vec::new();
    let mut expanded_cols = Vec::new();
    let mut row = 0;
    let mut col = 0;
    // if row empty, insert new row above
    while row < grid.len() {
        if grid[row].iter().all(|&c|c == '.') {
            if DEBUG { eprintln!("Expanding row {row}") };
            expanded_rows.push(row);
        }
        row += 1;
    }

    // if column empty, insert new column left
    while col < grid[0].len() {
        if grid.iter().map(|rv|rv[col]).all(|c|c=='.') {
            if DEBUG { eprintln!("Expanding col {col}") };
            expanded_cols.push(col);
        }
        col += 1;
    }

    let expansions = (expanded_rows, expanded_cols);

    if DEBUG { eprintln!("EXPANSIONS: {:?}", &expansions) };

    return expansions;
}

fn compute_distances(grid:&Vec<Vec<char>>) -> Vec<usize> {
    let gxy: Vec<(usize,usize)> = grid.iter().enumerate().map(
        |(r_idx, row)| row.iter().enumerate().filter_map(
            move |(c_idx, c)|(*c == '#').then(||(r_idx,c_idx))
        )
    ).flatten().collect();

    if DEBUG { eprintln!("FOUND GALAXIES: [{gxy:?}]") };

    let mut dist = Vec::new();
    // loop through every pair
    for (n,g) in gxy.iter().enumerate() {
        for g2 in gxy.iter().skip(n+1) {
            let gdist = g.0.abs_diff(g2.0) + g.1.abs_diff(g2.1);
            if DEBUG { eprintln!("{g:?} -> {g2:?}: {gdist:?}") };
            dist.push(gdist);
        }
    }
    
    return dist;
}

fn compute_distances_expanded(grid:&Vec<Vec<char>>, factor:usize, expansions:&(Vec<usize>,Vec<usize>)) -> Vec<usize> {
    let gxy: Vec<(usize,usize)> = grid.iter().enumerate().map(
        |(r_idx, row)| row.iter().enumerate().filter_map(
            move |(c_idx, c)|(*c == '#').then(||(r_idx,c_idx))
        )
    ).flatten().collect();

    if DEBUG { eprintln!("FOUND GALAXIES: [{gxy:?}]") };

    let mut dist = Vec::new();
    // loop through every pair
    for (n,g) in gxy.iter().enumerate() {
        for g2 in gxy.iter().skip(n+1) {
            let mut gdist = g.0.abs_diff(g2.0) + g.1.abs_diff(g2.1); 
            gdist += (factor-1)*expansions.0.iter().filter(|&&x|x < usize::max(g.0, g2.0) && x > usize::min(g.0,g2.0)).count();
            gdist += (factor-1)*expansions.1.iter().filter(|&&x|x < usize::max(g.1, g2.1) && x > usize::min(g.1,g2.1)).count();
            if DEBUG { eprintln!("{g:?} -> {g2:?}: {gdist:?}") };
            dist.push(gdist);
        }
    }
    
    return dist;
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // galactic observatory
    // puzzle input, star map of galaxies
    let mut lines = BufReader::new(input).lines();

    let mut grid = lines.map(|line|line.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    if DEBUG { eprintln!("grid: {:?}", Grid::from(&grid)) };

    // expand the universe
    let expansions = expand_old_universe(&mut grid);

    if DEBUG { eprintln!("expanded: {:?}", Grid::from(&grid)) };

    // compute the distances
    let dist = compute_distances_expanded(&grid, 1_000_000, &expansions);

    if DEBUG { eprintln!("dist: {:?}", &dist) };

    // find the farthest
    let steps:usize = dist.iter().sum();

    // PART TWO
    eprintln!("PART TWO");

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

#[test]
fn example() -> Result<(),Error> {
    let testinput = 
r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = 
r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    let mut grid = testinput.split_ascii_whitespace().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let testinput2 = 
r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
    let grid2 = testinput2.split_ascii_whitespace().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    expand_universe(&mut grid);

    assert_eq!(Grid::from(&grid), Grid::from(&grid2));
}

#[test]
fn testinput2() {
    let testinput = 
r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    let grid = testinput.split_ascii_whitespace().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let check_dist = compute_distances(&grid);

    eprintln!("{:?}", &check_dist);
}

#[test]
fn testinput3() {
    let testinput = 
r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    
    let grid = testinput.split_ascii_whitespace().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let expansions = expand_old_universe(&grid);

    let check_dist = compute_distances_expanded(&grid, 1, &expansions);

    eprintln!("Factor 1 {:?}", &check_dist);

    let steps:usize = check_dist.iter().sum();

    eprintln!("{steps}");

    let check_dist = compute_distances_expanded(&grid, 10, &expansions);

    eprintln!("Factor 10 {:?}", &check_dist);

    let steps:usize = check_dist.iter().sum();

    eprintln!("{steps}");

    let check_dist = compute_distances_expanded(&grid, 100, &expansions);

    eprintln!("Factor 100 {:?}", &check_dist);

    let steps:usize = check_dist.iter().sum();

    eprintln!("{steps}");
}
