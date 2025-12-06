const MAX: u32 = 1000; // compile-time value

fn main() {
    println!("The value of MAX is {MAX}.");

    // shadowing constants
    let max = MAX + 50; // max is bound to the memory that holds 50 + 1000
    println!("The value of max is {}.", {max});

    let x = "length";
    println!("The value of x is {x}.");

    {
        // shadowing vars
        let x = x.len(); // the above x is shadowed by this one so it is hidden or inaccessible in this SCOPE.
        println!("The value of x is {x}.");
    }
    
    println!("The value of x is {x}."); // should be 'length' since this is out of the shadowing scope

}
