use std::mem;

fn main() {
    // 4 + 8 + 1 = 13 for values.
    // Rust pads the tuple's memory to 16 bytes b/c of alignment..
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup.0 = {}\ntup.1 = {}\ntup.2 = {}", tup.0, tup.1, tup.2);
    println!();
    println!("x = {}\ny = {}\nz = {}", x, y, z);
    println!();
    println!("size of tup = {}", mem::size_of_val(&tup));
    println!("size of x = {}", mem::size_of_val(&x));
    println!("size of y = {}", mem::size_of_val(&y));
    println!("size of z = {}", mem::size_of_val(&z));
}
