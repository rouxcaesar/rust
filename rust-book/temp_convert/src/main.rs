use std::io;

fn main() {
    let mut temp = String::new();
    let mut unit = String::new();
    let conversion: f32;

    println!("What's your temperature value?");
    io::stdin()
        .read_line(&mut temp)
        .expect("Input should be a float");

    let value: f32 = match temp.trim().parse() {
       Ok(value) => value,
       Err(_) => panic!("Input should be a float"),
    };

    println!("What's your unit? (F or C)");
    io::stdin()
        .read_line(&mut unit)
        .expect("Input should be a string");

    let byte: &str = unit.trim(); 

    if byte == "F" {
        conversion = f_to_c(value);
        println!("Converted temperature: {}C", conversion);
    } else if byte == "C" {
        conversion = c_to_f(value);
        println!("Converted temperature: {}F", conversion);
    } else {
        panic!("Incorrect unit, please enter either F or C");
    }
}

fn f_to_c(t: f32) -> f32 {
   (t - 32.0) * (5.0/9.0) 
}

fn c_to_f(t: f32) -> f32 {
    (t * (9.0/5.0)) + 32.0
}
