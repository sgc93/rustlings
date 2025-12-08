use std::io;

fn degree_c_to_f() {
    println!("Converting Degree Celsius to Fehrenheit");

    let deg_c = loop {
        let mut deg_c = String::new();
        println!("Enter Temperatures in degree C: ");
        
        io::stdin()
            .read_line(&mut deg_c)
            .expect("Faild to read your entry.");

        let deg_c: f64 = match deg_c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter degree C in numeric value");
                continue;
            }
        };

        break deg_c;
    };

    let deg_f = (9.0 / 5.0) * deg_c + 32.0;

    println!("-----------------------\n{deg_c} deg C = {deg_f} deg F")
}

fn degree_f_to_c() {
    println!("Converting Degree Fehrenheit to Celsius");

    let deg_f = loop {
        let mut deg_f = String::new();
        println!("Enter Temperatures in degree F: ");

        io::stdin()
            .read_line(&mut deg_f)
            .expect("Faild to read your entry.");

        let deg_f: f64 = match deg_f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter degree F in numeric value");
                continue;
            }
        };

        break deg_f;
    };

    let deg_c = (5.0 / 9.0) * (deg_f - 32.0);

    println!("-----------------------\n{deg_f} deg F = {deg_c} deg C")
}

fn get_choice() -> u32 {
    const ERR_MSG: &str = "!! Enter correct choice, 1 or 2.";

    loop {
        println!("Choose action: \n  1 - Convert F to C \n  2 - Convert C to F");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("!! Failed to read your choice!");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{ERR_MSG}");
                continue;
            }
        };

        if choice == 1 {
            break choice;
        } else if choice == 2 {
            break choice;
        } else {
            println!("{ERR_MSG}");
            continue;
        }
    }
}

fn main() {
    println!("Starting ...");
    let choice = get_choice();
    if choice == 1 {
        degree_f_to_c();
    } else {
        degree_c_to_f();
    }
}
