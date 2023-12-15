use std::{io::{BufRead, BufReader, Cursor}, path, fmt::{Display, Write}, ops::Index};

use anyhow::{Error, Context};

const DEBUG:bool = cfg!(debug_assertions);

fn find_reflections(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let mut retv = Vec::new();
    'check:
    for c_col in 1..grid[0].len() {
        // check all the rows to see if this column works
        for c_row in 0..grid.len() {
            let mut test_col = 0;
            while (test_col < c_col) && (test_col + c_col) < grid[0].len() {
                if grid[c_row][c_col - test_col - 1] != grid[c_row][c_col + test_col] {
                    if DEBUG { eprintln!("Rejecting column {c_col} in row {c_row}")}
                    continue 'check; // not mirrored
                }
                test_col += 1;
            }
        }
        if DEBUG { eprintln!("Symmetry in column {c_col}")};
        retv.push(c_col);
    }

    return retv;
}

fn find_reflections_horiz(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let mut retv = Vec::new();
    'check:
    for c_row in 1..grid.len() {
        // check all the rows to see if this column works
        for c_col in 0..grid[0].len() {
            let mut test_row = 0;
            while (test_row < c_row) && (test_row + c_row) < grid.len() {
                if grid[c_row - test_row - 1][c_col] != grid[c_row + test_row][c_col] {
                    if DEBUG { eprintln!("Rejecting row {c_row} in column {c_col}")}
                    continue 'check; // not mirrored
                }
                test_row += 1;
            }
        }
        if DEBUG { eprintln!("Symmetry in row {c_row}")};
        retv.push(c_row);
    }

    return retv;
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
    // lava mirroring
    // puzzle input, grid of ash '.' and rocks '#'
    // let mut lines = BufReader::new(input).lines();

    // find lines of symmetry
    let mut summary1:usize = 0;
    let mut summary2:usize = 0;
    while let Ok(mut grid) = load_grid(input) {
        if DEBUG { eprintln!("grid: {:?}", Grid::from(&grid)) };
        let mut v = find_reflections(&grid);
        let mut h = find_reflections_horiz(&grid);
        summary1 += v.iter().copied().sum::<usize>();
        summary1 += h.iter().map(|&x|x*100).sum::<usize>();

        // part two, do the smudge checking.
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                // smudge
                if DEBUG { eprintln!("checking smudge ({row},{col}): ")}
                let old = grid[row][col];
                grid[row][col] = match old {
                    '.' => '#',
                    '#' => '.',
                    _ => panic!("bad grid"),
                };
                let mut t = find_reflections(&grid);
                t.retain(|x|!v.contains(x));
                if t.len() == 1 {
                    if DEBUG { eprintln!("########### NEW symmetry in col {}", t[0])}
                    // add this one so it won't get double-counted
                    v.push(t[0]);
                    summary2 += t[0];
                }
                let mut t = find_reflections_horiz(&grid);
                t.retain(|x|!h.contains(x));
                if t.len() == 1 {
                    if DEBUG { eprintln!("########### NEW symmetry in row {}", t[0])}
                    // add this one so it won't get double-counted
                    h.push(t[0]);
                    summary2 += 100 * t[0];
                }
                // restore smudge as real
                grid[row][col] = old;
            }
        }
    }

    println!("{summary1}");

    // PART TWO
    eprintln!("PART TWO");

    // find smudges

    // output the new symmetries
    println!("{summary2}");

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
r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    go(&mut testinput.as_bytes())
}

#[test]
fn example2() -> Result<(),Error> {
    let testinput = 
r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    go(&mut testinput.as_bytes())
}

#[test]
fn example3() -> Result<(),Error> {
    let testinput = 
r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    go(&mut testinput.as_bytes())
}

#[test]
fn testinput1() {
    let testinput = "...##.....";

    let mut grid = load_grid(&mut testinput.as_bytes()).unwrap();

    assert_eq!(find_reflections(&grid), vec![1,4,8,9]);
}

#[test]
fn testinput2() {
    let testinput = ".##.....";

    let mut grid = load_grid(&mut testinput.as_bytes()).unwrap();

    assert_eq!(find_reflections(&grid), vec![2,6,7]);
}


#[test]
fn testinput3() {
    let testinput = 
r"....#........
....#........";

    let mut grid = load_grid(&mut testinput.as_bytes()).unwrap();

    assert_eq!(find_reflections_horiz(&grid), vec![1]);
}

#[test]
fn testinput4() {
    let testinput = 
r"....#........

....#........";

    let mut b = Cursor::new(testinput);
    load_grid(&mut b).unwrap();
    load_grid(&mut b).unwrap();
}
