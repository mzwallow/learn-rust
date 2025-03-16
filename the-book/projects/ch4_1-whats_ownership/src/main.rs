fn main() {
    // Variable live in the stack
    let a = 5;
    let mut _b = a;
    _b += 1;
    // NOTE: Frames in stack are associated with specific funciton,
    // and are deallocated when the function returns.

    // Boxes live in the heap
    let a = [0; 1_000_000];
    let _b = a; // b is copied array

    let a = Box::new([0; 1_000_000]); // Pointer
    let _b = a; // Pointee
    // NOTE: Heap data is not tied to a specific stack frame.

    // NOTE: Memory management
    //
    // If a variable owns a box, when Rust deallocates the variable’s frame,
    // then Rust deallocates the box’s heap memory.

    // Variable cannot be used after being moved
    let first = String::from("Ferris");
    let _full = add_suffix(first);
    // println!("{_full}, originally {first}"); // <- undefined behavior: pointer used after its pointee is freed
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
