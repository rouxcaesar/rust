fn main() {
    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &s;

    println!("{} and {}", s1, s2);

    let s3 = &mut s;
    println!("{}", s3);

    let s4 = &mut s;
    println!("{}", s4);
}
