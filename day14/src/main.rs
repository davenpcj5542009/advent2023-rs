use std::{io::{BufRead, BufReader}, fmt::Write};


use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

fn roll_rocks(direction: isize, grid: &mut Vec<Vec<char>>) -> usize {
    let mut load = 0;

    let mut tops = Vec::new();
    tops.resize(grid[0].len(), 0);

    let mut row = 0;
    while row < grid.len() {
        let mut col = 0;
        while col < grid[row].len() {
            if grid[row][col] == 'O' {
                if DEBUG { eprint!("({row},{col})") }
                let nr = tops[col];
                if nr < row {
                    if DEBUG { eprintln!(", Rolling to {nr}")}
                    grid[nr][col] = 'O';
                    grid[row][col] = '.';
                    tops[col] = nr+1;
                    // measure the load
                    load += grid.len() - nr;
                } else {
                    if DEBUG { eprintln!(", Stopped")}
                    load += grid.len() - row;
                    tops[col] = row+1;
                }
            } else if grid[row][col] == '#' {
                if DEBUG { eprintln!("({row},{col}) Fixed")}
                tops[col] = row+1;
            }
            col += 1;
        }
        row += 1;
    }
    if DEBUG { eprintln!("rolled grid: {:?}\nload[{load}]", Grid::from(grid.as_ref())) };

    return load;
}

fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // rock rolling
    // puzzle input, rock positions - 'O' rolls, '#' fixed
    let mut lines = BufReader::new(input).lines();

    let mut grid = lines.map(|line|line.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    if DEBUG { eprintln!("grid: {:?}", Grid::from(&grid)) };

    // roll rocks
    let load = roll_rocks(-1, &mut grid);

    if DEBUG { eprintln!("rolled grid: {:?}", Grid::from(grid.as_ref())) };
  
    println!("load: {:?}", &load);

    // PART TWO
    // eprintln!("PART TWO");

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
r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    go(&mut testinput.as_bytes())
}

