fn main() {
    mutability();
    constants();
    shadowing();
}

fn mutability() {
    println!("Mutability:");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("");
}

// NOTE:
// - Aren't allowed to use `mut`
// - MUST be annotated
// - Can be declared in ANY scope
const ONE_HOURS_IN_SECONDS: u32 = 60 * 60 * 1;
fn constants() {
    println!("Constants:");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("ONE_HOURS_IN_SECONDS: {ONE_HOURS_IN_SECONDS}");
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    println!("");
}

fn shadowing() {
    println!("Shadowing:");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("");
}
