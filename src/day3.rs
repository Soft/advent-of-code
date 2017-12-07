const INPUT: i32 = 368078;

#[inline]
fn area(n: i32) -> i32 {
    4*n.pow(2) - 4*n + 1
}

fn coords(a: i32, n: i32, i: i32) -> (i32, i32) {
    let s = a/4;
    let (c1, c2, c3, c4) = (s - 1, s*2 - 1, s*3 - 1, s*4 - 1);
    if i <= c1 {
        (n, i - s/2)
    } else if c1 < i && i <= c2 {
        (i - s/2 - s, n)
    } else if c2 < i && i <= c3 {
        (-n, i - s/2 - 2*s)
    } else {
        (i - s/2 - 3*s, -n)
    }
}

fn main() {
    let layer = (1..)
        .find(|a: &i32| INPUT <= area(*a))
        .unwrap();
    let a1 = area(layer-1);
    let a2 = area(layer);
    let cells = a2 - a1; // Cells in the current layer
    let index = INPUT - a1; // Offset within the layer
    let (x, y) = coords(cells, layer - 1, index);
    let dist = x.abs() + y.abs();
    println!("{}", dist);
}


