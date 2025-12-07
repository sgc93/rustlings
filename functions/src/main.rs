use std::io;

fn display_data(name: &str, initial: char){
    print!("Your name is {name}, so your initial is {initial}.");
}

fn get_initials(name: &str) -> char {
    let mut chars = name.chars();

    chars.next().unwrap()
}

fn get_name() -> String {
    let mut name = String::new();
    
    println!("Please enter your full name");
    io::stdin().read_line(&mut name).expect("Failed to read your name");

    name
}

fn main() {
    println!("Starting ...");
    let name = get_name();
    let initial = get_initials(&name);
    display_data(&name, initial);
}
