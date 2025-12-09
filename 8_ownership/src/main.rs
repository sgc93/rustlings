fn display_title(title: &str) {
    println!("My title is {}", title)
} // title will not dropped the value because it doesn't own it -> it just borrows

fn make_name_full(title: &mut String) {
    title.push_str(" Too IV");

    // display_full_name(title);
}

fn display_full_name(name: &mut String) {
    name.push_str(" or Dr Too the fourth");

    println!("My full name is: {}.", name);
}

fn main() {
    // Heap Allocation
    {
        let str1 = String::from("Hello"); // Allocation: str1 pointer in the stack by CPU and the value "Hellow" in the heap by allocator
        let str2 = str1; // str1 is no longer available -> moved to str2 | ownership (of pointer to heep that holds the actual value) transfer -> dropped from stack by the RUST

        // println!("Str1: {}", str1); // Error: trying to borrow moved value
        println!("Str2: {}", str2);
    } // str2 Dropped - it's no more valid -> happened by the rust+type at language level -> programatically with drop trait like RAII in c++
    // No DOUBLE FREE! since str1 is already dropped as soon as it is moved to str2

    // Read-only static/program memory
    {
        let str1 = "PI=3.14"; // Allocation: str1 borrowed reference on stack and points value store in read-only program memory when the program starts -> value is stored on neither stack nor heap
        let str2 = str1; // real-copy - both str1 and str2 reference the same value in the read-only static memory

        println!("Str1: {}", str1); // Str1: PI=3.14
        println!("Str2: {}", str2); // Str1: PI=3.14
    } // Both str1 and str dropped from stack -> BUT the string value stays forever (as long as the program runs)

    // Stack Allocation
    {
        let x = 3.14; // Allocation: on stack since it has fixed compile time size -> stored exacly as x=3.14
        let y = x; // real copy | Bit-by-bit duplication -> both x and y binds to 3.14 on the stack independently

        println!("x: {x}."); // x: 3.14.
        println!("y: {y}."); // y: 3.14.
    } // Both x and y are Deallocated! (not dropped - no destructor) -> stack pointer moves back -> happes in cpu automatically at hardware level

    // Borrowing
    {
        let mut title = String::from("Dr");
        display_title(&title);
        println!("Title: {}", title);

        make_name_full(&mut title);
        display_full_name(&mut title);

        println!("Full name: {}", title);
    } // title ends here and droped the heap value since it owns it
}

/*
Solutions in Rust for Memory-freeing Bugs
'''''''''''''''''''''''''''''''''''''''''
1. Double Free -> "There is only a single owner of a heap data"
2. Use After Free | Dangling Pointer -> "A value cannot be freed/dropped while references/borrowers to it exists"
    let s = String::from("Hello");
    let r = &s;
    drop(s); // Compile time error since r borrows s
    println!("{}", r);
3. Memory Leak | Never Freeing Memory -> "Every owned value in the heap is dropped exacly once at the end of the scope"
4. Invalid Free -> "Only owner can free a memory, and via Drop"
*/
