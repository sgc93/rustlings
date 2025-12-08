use std::io;

fn calc_nth(n: u32) {
    println!("Calculating {n}th fibunacci sequence number ...");

    let mut a = 1; // F1
    let mut b = 1; // F2

    for _ in 3..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    println!("The {n}th number in Fibonacci sequence is {b}.")
}

fn get_n() -> u32 {
    loop {
        println!("Fibonacci sequence: n1 = 1, n2 = 1. Enter n, where n is an integer > 2.");
        println!("n=");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("!! Failed to read n!");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("!! Please enter integer value.");
                continue;
            }
        };

        if n > 2 {
            break n;
        } else {
            println!("!! Please enter integer > 2");
            continue;
        }
    }
}

fn main() {
    println!("Starting ...");
    let n = get_n();
    calc_nth(n);
}
