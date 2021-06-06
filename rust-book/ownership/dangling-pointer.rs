fn main() {
    let ref_to_nothing = dangle();
    println!("{}", ref_to_nothing);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}
