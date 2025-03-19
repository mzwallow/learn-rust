use std::vec;

fn main() {
    //==========================================================================
    // Move
    //
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    moved_fn(m1, m2); // Ownership's moved to g1 and g2
    // let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved

    //==========================================================================
    // References Are Non-Owning Pointers (Borrow)
    //
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    borrowed_fn(&m1, &m2);
    let _s = format!("{} {}", m1, m2);

    //==========================================================================
    // Dereference
    //
    let mut x: Box<i32> = Box::new(1);
    let _a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value, so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let _b: i32 = **r1; // two dereferences get us to the heap value

    let r2: &i32 = &*x; // r2 points to the heap value directly
    let _c: i32 = *r2; // so only one dereference is needed to read it
    //
    // NOTE: Rust implicitly inserts dereferences and references in certain cases,
    // such as calling a method with the dot operator.

    //==========================================================================
    // Rust Avoids Simultaneous Aliasing and Mutation
    //
    // NOTE: Pointer Safety Principle: data should never be aliased and mutated at the same time.
    let mut v: Vec<i32> = vec![1, 2, 3];
    let _num: &i32 = &v[2]; // Borrowed
    v.push(4); // Mutated, vector create new allocation due to it's at cap.
    // println!("Third element is {}", *_num); // ERROR: undefined behavior: used after free

    //==========================================================================
    // References Change Permissions on Places
    //
    // NOTE: Data has their own 3 permission (within compiler): Read, Write, and Own
    let _v: Vec<i32> = vec![1, 2, 3]; // v: R-O
    let mut v: Vec<i32> = vec![1, 2, 3]; // v: RWO

    let num: &i32 = &v[2];
    // v    : R--
    // num  : R-O
    // *num  : R--

    println!("Third element is {}", *num);
    // v    : RWO
    // num  : ---
    // *num : ---

    v.push(4); // v: ---

    //==========================================================================
    // The Borrow Checker Finds Permission Violations
    //
    let mut _v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &_v[2];
    // _v.push(4); // ERROR: required RW- but got only R--
    println!("Third element is {}", *num);

    //==========================================================================
    // Mutable References Provide Unique and Non-Owning Access to Data
    //
    // immutable references (shared references)
    // mutable references (unique references)
    //
    let mut v: Vec<i32> = vec![1, 2, 3]; // v: RWO

    let num: &mut i32 = &mut v[2];
    // v    : ---
    // num  : R-O
    // *num : RW-

    *num += 1;
    println!("Third element is {}", *num);
    // v    : RWO
    // num  : ---
    // *num : ---

    println!("Vector is now {:?}", v); // v: ---
    // NOTE: Mutable references can also be temporarily “downgraded” to read-only references.
    // let num2: &i32 = &*num; // *num: R--

    //==========================================================================
    // Permissions Are Returned At The End of a Reference’s Lifetime
    //
    fn _ascii_capitalize(v: &mut Vec<char>) {
        let c = &v[0];
        // v : R--
        if c.is_ascii_lowercase() {
            let up = c.to_ascii_uppercase();
            // v : RW-
            v[0] = up;
        } else {
            // v : RW-
            println!("Already capitalized: {:?}", v);
        }
    }

    //==========================================================================
    // Data Must Outlive All Of Its References
    //
    let _s = String::from("Hello world");
    let s_ref = &_s;
    // s : R--

    // drop(s); // ERROR: required R-O but got only R--
    println!("{}", s_ref);

    // New F (flow) permission
    // Safe
    fn _first(strings: &Vec<String>) -> &String {
        let s_ref = &strings[0]; // &strings[0] required R--_F
        s_ref // R--_F
    }
    // Unsafe
    /*
        // ERROR: Lack lifetime param
        fn first_or(strings: &Vec<String>, default: &String) -> &String {
            let s_ref = &strings[0];
            if *s_ref != "" { s_ref } else { default }
        }
        let strings = vec![];
        let default = String::from("default");
        let s = first_or(&strings, &default);
        drop(default); // could invalidate s
        println!("{}", s);
    */
}

fn moved_fn(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

fn borrowed_fn(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}
