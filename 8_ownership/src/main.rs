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
}
