use std::io;

fn check_length(password: &str) -> bool {
    password.len() >= 8
}

fn check_number(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_digit())
}

fn check_cap(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_uppercase())
}

fn check_lowercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_lowercase())
}

fn check_symbol(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_punctuation())
}

fn check_password(password: String) {
    println!("Checking ...");
    let is_length_strong = check_length(&password);
    if is_length_strong {
        println!("✅ Strong length.")
    } else {
        println!("❌ Weak length.")
    }

    let contain_number = check_number(&password);
    if contain_number {
        println!("✅ Contain Number(s).")
    } else {
        println!("❌ Doesn't Contain Number.")
    }

    let contain_lowers = check_lowercase(&password);
    if contain_lowers {
        println!("✅ Contain Lowercase(s).")
    } else {
        println!("❌ Doesn't Contain Lowercases.")
    }

    let contain_caps = check_cap(&password);
    if contain_caps {
        println!("✅ Contain Uppercase(s).")
    } else {
        println!("❌ Doesn't Contain Uppercases.")
    }

    let contain_symbol = check_symbol(&password);
    if contain_symbol {
        println!("✅ Contain Symbol(s).")
    } else {
        println!("❌ Doesn't Contain Symbols.")
    }

    // scoring
    let _score = [is_length_strong, contain_caps, contain_lowers, contain_number, contain_symbol].iter().filter(|&&p| p).count();
    match _score {
        5 => println!("\n..............................\n🔥 Strong Password!"),
        3..=4 => println!("\n..............................\n🟡 Medium Strength Password!"),
        _ => println!("\n..............................\n❌ Bro ths isn't Password!"),
    }
}

fn get_password() -> String {
    let mut password = String::new();
    println!("Enter Password: ");

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read password.");

    return password.trim().to_string();
}

fn main() {
    let password = get_password();
    check_password(password);
}
