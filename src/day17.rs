const INPUT: usize = 301;

fn main() {
    let mut ind: usize = 0;
    let mut buffer = Vec::with_capacity(2017);
    buffer.push(0);

    for n in 1..2018 {
        ind = (ind + INPUT) % buffer.len() + 1;
        buffer.insert(ind, n);
    }

    let last = (ind + 1) % buffer.len();

    println!("{}", buffer[last]);

    ind = 0;
    let mut zero_ind = 0;
    let mut result = None;
    let mut len = 1;

    for n in 1..50_000_000+1 {
        ind = ((ind + INPUT) % len) + 1;
        if ind <= zero_ind {
            zero_ind += 1;
        } else if ind == (zero_ind+1) % len {
            result = Some(n);
        }
        len += 1;
    }

    println!("{}", result.unwrap());
}

