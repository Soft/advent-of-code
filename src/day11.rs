use std::str::FromStr;

const INPUT: &str = include_str!("input/day11.txt");

type Coords = (i32, i32, i32);

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    N, NE, SE, S, SW, NW
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "n" => Ok(Direction::N),
            "ne" => Ok(Direction::NE),
            "se" => Ok(Direction::SE),
            "s" => Ok(Direction::S),
            "sw" => Ok(Direction::SW),
            "nw" => Ok(Direction::NW),
            _ => Err(())
        }
    }
}

fn distance((y1, x1, z1): Coords, (y2, x2, z2): Coords) -> i32 {
    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) / 2
}

fn main() {
    let coords = INPUT
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .fold((0, 0, 0), |(y, x, z), d| {
            match d {
                Direction::N  => (y+1, x,   z-1),
                Direction::NE => (y,   x+1, z-1),
                Direction::SE => (y-1, x+1, z),
                Direction::S  => (y-1, x,   z+1),
                Direction::SW => (y,   x-1, z+1),
                Direction::NW => (y+1, x-1, z)
            }
        });
    println!("{}", distance((0, 0, 0), coords));

}


