fn who_am_i() -> String{
    return "I am Rustacean.".to_string();
}

fn main() {
    println!("Hello, world!");
    println!("My name is {0}, {1} {0}.", "G", "Smex");
    println!("{me}", me = (who_am_i()));
}
