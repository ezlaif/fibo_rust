use std::mem;

fn main() {
    fibonacci(222);
}

fn fibonacci(limit: i64) {
    let mut sum: i64 = 0;
    let mut current: i64 = 1;
    let mut counter: i64 = 0;

    while counter < limit  {
        counter += 1;
        sum += current;
        mem::swap(&mut sum, &mut current);
        println!("{} ", sum);
    }
}