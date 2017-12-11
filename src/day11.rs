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
            "n"  => Ok(Direction::N),
            "ne" => Ok(Direction::NE),
            "se" => Ok(Direction::SE),
            "s"  => Ok(Direction::S),
            "sw" => Ok(Direction::SW),
            "nw" => Ok(Direction::NW),
            _    => Err(())
        }
    }
}

fn distance((y1, x1, z1): Coords, (y2, x2, z2): Coords) -> i32 {
    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) / 2
}

fn main() {
    let (coords, max) = INPUT
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .fold(((0, 0, 0), i32::min_value()), |((y, x, z), max), d| {
            let coords = match d {
                Direction::N  => (y+1, x,   z-1),
                Direction::NE => (y,   x+1, z-1),
                Direction::SE => (y-1, x+1, z),
                Direction::S  => (y-1, x,   z+1),
                Direction::SW => (y,   x-1, z+1),
                Direction::NW => (y+1, x-1, z)
            };
            let dist = distance((0, 0, 0), coords);
            let max = if dist > max { dist } else { max };
            (coords, max)
        });
    println!("{}", distance((0, 0, 0), coords));
    println!("{}", max);
}


