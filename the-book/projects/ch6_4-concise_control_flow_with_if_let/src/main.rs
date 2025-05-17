fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // Change above code to this:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // We can also use `else` that behave like `_ => ()` in `match`
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    // Quiz 1
    enum Location {
        Point(i32),
        Range(i32, i32),
    }
    fn print_range_max(loc: &Location) {
        // print the second field of Range, if loc is a Range
        if let Location::Range(_, y) = loc {
            println!("The second field (y) of loc is: {y}!");
        }
    }
    let loc = Location::Range(3, 9);
    print_range_max(&loc);

    // Quiz 2
    fn get_start(loc: &Location) -> i32 {
        // return the first field of Range or the only field of Point
        match loc {
            Location::Range(x, _) => *x,
            Location::Point(x) => *x,
        }
    }
    let loc = Location::Point(11);
    println!("Result: {}!", get_start(&loc));
}
