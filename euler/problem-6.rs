fn main() {
    let max: u64 = 100;
    let sum_squares = sum_squares(max);
    let square_of_sum = square_of_sum(max);

    println!("result = {}", square_of_sum - sum_squares);
}

fn square_of_sum(n: u64) -> u64 {
    let mut sum: u64 = 0;

    for i in 1..=n {
        sum += i;
    }

    return sum * sum;
}

fn sum_squares(n: u64) -> u64 {
    let mut sum: u64 = 0;

    for i in 1..=n {
        sum += i * i;
    }

    return sum;
}
