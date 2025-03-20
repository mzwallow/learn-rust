use std::rc::Rc;

fn main() {
    fixing_unsafe_program();
    println!("=======");
    fixing_safe_program();
}

fn fixing_unsafe_program() {
    // =========================================================================
    // Returning a Reference to the Stack
    //

    /*  ERROR
        fn return_a_string() -> &String {
            let s = String::from("Hello world");
            &s
        }
    */

    // Move ownership out of function
    fn _return_a_string_1() -> String {
        let s = String::from("Hello world");
        s
    }
    // Return a string literal using `'static`, no heap allocation required
    fn _return_a_string_2() -> &'static str {
        "Hello world"
    }
    // Using "Reference-counted pointer" to defer borrow-checking to runtime by gc.
    // Rc::clone only clone a pointer to s and runtime will check last ref and then
    // deallocates the data.
    fn _return_a_string_3() -> Rc<String> {
        let s = Rc::new(String::from("Hello world"));
        Rc::clone(&s)
    }
    // Using a mutable reference
    fn _return_a_string_4(output: &mut String) {
        output.replace_range(.., "Hello world");
    }

    // =========================================================================
    // Not Enough Permissions
    //

    /*  ERROR
        fn stringify_name_with_title(name: &Vec<String>) -> String {
            name.push(String::from("Esq.")); // push required RW- but missing W
            let full = name.join(" ");
            full
        }
        // ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
        let name = vec![String::from("Ferris")];
        let first = &name[0];
        stringify_name_with_title(&name);
        println!("{}", first); // ERROR: push invalidated first
    */

    // Using mutable reference (NOT GOOD)
    // <Functions should not mutate their inputs if the caller would not expect it>
    fn _stringify_name_with_title_1(name: &mut Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join(" ");
        full
    }
    // Take ownership
    // <It is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String>
    fn _stringify_name_with_title_2(mut name: Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join(" ");
        full
    }
    // SO using &Vec is ACTUALLY GOOD WITH changing the body function
    fn _stringify_name_with_title_3(name: &Vec<String>) -> String {
        let mut name_clone = name.clone();
        name_clone.push(String::from("Esq."));
        let _full = name_clone.join(" ");
        // or
        let mut full = name.join(" "); // Better: join already copies the data into full
        full.push_str(" Esq.");

        full
    }

    // =========================================================================
    // Aliasing and Mutating a Data Structure
    //

    /*  ERROR
        fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
            let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
            for s in src {
                if s.len() > largest.len() {
                    dst.push(s.clone()); // ERROR: lost W and push could deallocate dst
                                         // invalidated largest
                }
            }
        }
    */

    // Shorten `largest` lifetime
    fn _add_big_strings_1(dst: &mut Vec<String>, src: &[String]) {
        // Could cause perfomance hit
        let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
        for s in src {
            if s.len() > largest.len() {
                dst.push(s.clone());
            }
        }
    }
    // Perform all length comparison first then mutate `dst` afterwards
    fn _add_big_strings_2(dst: &mut Vec<String>, src: &[String]) {
        let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
        let to_add: Vec<String> = src
            .iter()
            .filter(|s| s.len() > largest.len())
            .cloned()
            .collect();
        dst.extend(to_add);
    }
    // Copy length of `largest` first
    fn _add_big_strings_3(dst: &mut Vec<String>, src: &[String]) {
        let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
        for s in src {
            if s.len() > largest_len {
                dst.push(s.clone());
            }
        }
    }

    // =========================================================================
    // Copying vs. Moving Out of a Collection
    //

    /*
     * Safe
        let v: Vec<i32> = vec![0, 1, 2];
        let n_ref: &i32 = &v[0];
        let n: i32 = *n_ref;

     * Unsafe
        let v: Vec<String> = vec![String::from("Hello world")];
        let s_ref: &String = &v[0];
        let s: String = *s_ref;

        // These drops are normally implicit, but we've added them for clarity.
        drop(s);
        drop(v); // ERROR: double-free
    */
    // NOTE: If a value does not own heap data, then it can be copied without a move

    // Use immutable reference
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");
    // Clone
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");
    // Use `Vec::remove` to move the string out of vector
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}

fn fixing_safe_program() {
    // =========================================================================
    // Mutating Different Tuple Fields
    //

    /*
     * Safe
        let mut name = (String::from("Ferris"), String::from("Rustacean"));
        let first = &name.0;
        name.1.push_str(", Esq.");
        println!("{first} {}", name.1);

     * Unsafe
     *
     * Rust only look at the type signature `&(String, String)`
     * Above, `name.1.push_str` works fine. But for `_get_first` Rust decides that both
     * `name.0` and `name.1` get borrowed, and eliminates WO on both.
        let mut name = (String::from("Ferris"), String::from("Rustacean"));
        let first = _get_first(&name);
        name.1.push_str(", Esq.");
        println!("{first} {}", name.1);
    */

    // =========================================================================
    // Mutating Different Array Elements
    //

    // Safe
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    *x += 1;
    println!("{a:?}");

    // Unsafe
    // Rust use `a[_]` for all indexex of `a` so it lacks of R permission
    /*
        let mut a = [0, 1, 2, 3];
        let x = &mut a[1];
        let y = &a[2];
        *x += *y;
    */

    // Use `slice::split_at_mut` (use unsafe in underlying)
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
    // Use unsafe
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe {
        *x += *y;
    } // DO NOT DO THIS unless you know what you're doing!
}

fn _get_first(name: &(String, String)) -> &String {
    &name.0
}
