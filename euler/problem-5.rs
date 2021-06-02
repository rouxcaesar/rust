// Smallest multiple

fn main() {
    //let mut result: i32 = 0;
    let max: u64 = 300_000_000_000;  

    for i in 11..max {
        if !(i % 20 == 0) {
            continue;
        }
        if !(i % 19 == 0) {
            continue;
        }
        if !(i % 18 == 0) {
            continue;
        }
        if !(i % 17 == 0) {
            continue;
        }
        if !(i % 16 == 0) {
            continue;
        }
        if !(i % 15 == 0) {
            continue;
        }
        if !(i % 14 == 0) {
            continue;
        }
        if !(i % 13 == 0) {
            continue;
        }
        if !(i % 12 == 0) {
            continue;
        }
        if !(i % 11 == 0) {
            continue;
        }

        println!("result = {}", i);
        break;
    }
}
