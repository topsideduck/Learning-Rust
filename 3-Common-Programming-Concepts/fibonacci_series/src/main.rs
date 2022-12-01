use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter number of terms of Fibonacci sequence: ");

    io::stdin().read_line(&mut n).expect("Could not read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut t1: u32 = 0;
    let mut t2: u32 = 1;
    let mut temp: u32 = 0;

    for _i in 0..n {
        print!("{t1} ");
        temp = t2;
        t2 += t1;
        t1 = temp;
    }

    println!();
}
