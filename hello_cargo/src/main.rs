fn who_am_i() -> String{
    return "I am Rustacean".to_string();
}

fn main() {
    println!("Hello, world!");
    println!("{}", (who_am_i()));
}
