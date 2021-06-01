fn main() {
    let max = 10_000;
    let mut sum = 0;

    for i in 1..=max {
        let f = fib(i);
        if f % 2 == 0 {
            sum += f;
        }

        if f >= 4_000_000  {
            break;
        }
    }

    println!("sum = {}", sum);
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}
