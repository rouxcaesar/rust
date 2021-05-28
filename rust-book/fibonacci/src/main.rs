use std::io;

fn main() {
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Enter an int");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Wrong input!"),
        };

        let r = fib(n);
        println!("{}th number of the Fibonacci sequence: {}", n, r);
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fib(n-1) + fib(n-2);
}
