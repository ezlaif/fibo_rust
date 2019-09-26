use std::mem;

fn main() {
    fibonacci(92);
}

fn fibonacci(limit: u8) {
    let mut sum: u64 = 0;
    let mut current: u64 = 1;
    let mut counter: u8 = 0;

    while counter < limit  {
        counter += 1;
        sum += current;
        mem::swap(&mut sum, &mut current);
        println!("{} ", sum);
    }
}