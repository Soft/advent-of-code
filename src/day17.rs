const INPUT: usize = 301;

fn main() {
    let mut ind: usize = 0;
    let mut buffer = Vec::with_capacity(2017);
    buffer.push(0);

    for n in 1..2018 {
        ind = (ind + INPUT + 1) % buffer.len();
        buffer.insert(ind, n);
    }

    let last = (ind + 1) % buffer.len();
    println!("{}", buffer[last]);
    
}

