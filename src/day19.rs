const INPUT: &str = include_str!("input/day19.txt");

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum Dir { Up, Down, Left, Right }

type Point = (usize, usize);
type Grid = Vec<Vec<char>>;

use Dir::*;

fn continuation_dir(grid: &Grid, (x, y): Point, dir: Dir) -> (Point, Dir) {
    vec![((x, y-1), Up), ((x, y+1), Down), ((x-1, y), Left), ((x+1, y), Right)]
        .into_iter()
        .find(|&((x, y), d)| d != dir &&
              (grid[y][x] == '|' || grid[y][x] == '-'))
        .unwrap()
}

impl Dir {
    fn opposite(&self) -> Self {
        match *self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left
        }
    }
}

fn main() {
    let grid: Grid = INPUT
        .lines()
        .map(|str| str
             .chars()
             .collect())
        .collect();

    let start_x = grid[0]
        .iter()
        .position(|c| *c == '|')
        .unwrap();

    let (mut x, mut y) = (start_x, 0);
    let mut dir = Down;
    let mut letters: Vec<char> = vec![];

    loop {
        if grid[y][x].is_alphabetic() {
            letters.push(grid[y][x])
        }

        let (x2, y2) = match grid[y][x] {
            ' ' => break,
            '+' => {
                let (point, dir2) = continuation_dir(&grid, (x, y), dir.opposite());
                dir = dir2;
                point
            },
            _ => match dir {
                Up => (x, y-1),
                Down => (x, y+1),
                Left => (x-1, y),
                Right => (x+1, y),
            }
        };

        x=x2;
        y=y2;
    }

    println!("{}", letters.iter().collect::<String>());

}

