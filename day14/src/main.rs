use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Write;
use std::io::{BufRead, BufReader};
use std::hash::{Hash, Hasher};


use anyhow::{Error};

const DEBUG:bool = cfg!(debug_assertions);

fn compute_load(grid: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    let mut row = 0;
    while row < grid.len() {
        let mut col = 0;
        while col < grid[row].len() {
            if grid[row][col] == 'O' {
                load += grid.len() - row;
            }

            col += 1;
        }
        row += 1;
    }
    if DEBUG { eprintln!("load[{load}]") };

    return load;
}

fn roll_rocks_north(direction: isize, grid: &mut Vec<Vec<char>>) -> usize {
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

fn roll_rocks_west(grid: &mut Vec<Vec<char>>) {
    let mut stops = Vec::new();
    stops.resize(grid.len(), 0);

    let mut row = 0;
    while row < grid.len() {
        let mut col = 0;
        while col < grid[row].len() {
            if grid[row][col] == 'O' {
                let nc = stops[row];
                if nc < col {
                    grid[row][nc] = 'O';
                    grid[row][col] = '.';
                    stops[row] = nc+1;
                } else {
                    stops[row] = col+1;
                }
            } else if grid[row][col] == '#' {
                stops[row] = col+1;
            }
            col += 1;
        }
        row += 1;
    }

    if DEBUG { eprintln!("rolled grid west: {:?}", Grid::from(grid.as_ref())) };
}

fn roll_rocks_south(grid: &mut Vec<Vec<char>>) {
    let mut stops:Vec<isize> = Vec::new();
    stops.resize(grid[0].len(), grid.len() as isize-1);

    let mut row = grid.len() - 1;
    loop {
        let mut col = 0;
        while col < grid[row].len() {
            if grid[row][col] == 'O' {
                let nr = stops[col];
                if nr > row as isize {
                    grid[nr as usize][col] = 'O';
                    grid[row][col] = '.';
                    stops[col] = nr as isize-1;
                } else {
                    stops[col] = row as isize-1;
                }
            } else if grid[row][col] == '#' {
                stops[col] = row as isize-1;
            }
            col += 1;
        }
        if row == 0 {
            break;
        }
        row -= 1;
    }
    if DEBUG { eprintln!("rolled grid south: {:?}", Grid::from(grid.as_ref())) };
}

fn roll_rocks_east(grid: &mut Vec<Vec<char>>) {
    let mut stops:Vec<isize> = Vec::new();
    stops.resize(grid.len(), grid[0].len() as isize-1);

    let mut row = 0;
    while row < grid.len() {
        let mut col = grid[row].len() - 1;
        loop {
            if grid[row][col] == 'O' {
                let nc = stops[row];
                if nc > col as isize {
                    grid[row][nc as usize] = 'O';
                    grid[row][col] = '.';
                    stops[row] = nc-1;
                } else {
                    stops[row] = col as isize-1;
                }
            } else if grid[row][col] == '#' {
                stops[row] = col as isize-1;
            }
            if col == 0 {
                break;
            }
            col -= 1;
        }
        row += 1;
    }

    if DEBUG { eprintln!("rolled grid east: {:?}", Grid::from(grid.as_ref())) };
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn roll_rocks(cycles:usize, grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut pgrid = calculate_hash(&grid);
    let mut cache:HashMap<u64, Vec<Vec<char>>> = HashMap::new();
    let mut result_cache = HashMap::new();
    cache.insert(pgrid, grid);
    for _cycle in 1..=cycles {
        pgrid = *result_cache.entry(pgrid).or_insert_with(|| {
                let mut newgrid = cache.get(&pgrid).unwrap().clone();

                roll_rocks_north(0,&mut newgrid);
                roll_rocks_west(&mut newgrid);
                roll_rocks_south(&mut newgrid);
                roll_rocks_east(&mut newgrid);
                let newhash = calculate_hash(&newgrid);
                if let Some(oldgrid) = cache.insert(newhash, newgrid) {
                    println!("hash collision");
                    assert_eq!(Some(&oldgrid), cache.get(&newhash));
                };
                newhash
            }
        );
    }

    return cache.get(&pgrid).unwrap().clone();
}


fn go(input:&mut dyn BufRead) -> Result<(),Error>{
    // rock rolling
    // puzzle input, rock positions - 'O' rolls, '#' fixed
    let mut lines = BufReader::new(input).lines();

    let mut grid = lines.map(|line|line.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    if DEBUG { eprintln!("grid: {:?}", Grid::from(&grid)) };

    // roll rocks
    let load = roll_rocks_north(-1, &mut grid.clone());

    if DEBUG { eprintln!("rolled grid: {:?}", Grid::from(grid.as_ref())) };
  
    println!("load: {:?}", &load);

    // PART TWO
    eprintln!("PART TWO");

    let grid = roll_rocks(1_000_000_000, grid);
    let load = compute_load(&grid);

    println!("load: {:?}", &load);

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

#[test]
fn example2() -> Result<(),Error> {
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

#[test]
fn testinput1() -> Result<(),Error> {
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

    let mut grid = testinput.lines().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let grid = roll_rocks(1, grid);

    if DEBUG { eprintln!("cycled grid: {:?}", Grid::from(&grid)) };
    Ok(())
}

#[test]
fn testinput2() -> Result<(),Error> {
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

    let mut grid = testinput.lines().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let grid = roll_rocks(2, grid);

    if DEBUG { eprintln!("cycled grid: {:?}", Grid::from(&grid)) };
    Ok(())
}

#[test]
fn testinput3() -> Result<(),Error> {
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

    let mut grid = testinput.lines().map(|line|line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let grid = roll_rocks(3, grid);

    if DEBUG { eprintln!("cycled grid: {:?}", Grid::from(&grid)) };
    Ok(())
}