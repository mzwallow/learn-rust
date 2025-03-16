fn main() {
    scalar_types();
    compound_types();
}

fn scalar_types() {
    println!("Integer:");
    println!(
        "
        Length\tSigned\tUnsigned
        8-bit\ti8\tu8
        16-bit\ti16\tu16
        32-bit\ti32\tu32
        64-bit\ti64\tu64
        128-bit\ti128\tu128
        arch\tisize\tusize

        i: -(2^(n-1)) to 2^(n-1) - 1
        u: 0 to 2^n - 1
        "
    );

    println!("Integer literals:");
    println!(
        "
        Number literals\tExample
        Decimal\t\t98_222
        Hex\t\t0xff
        Octal\t\t0o77
        Binary\t\t0b1111_0000
        Byte (u8 only)\tb'A'
        "
    );

    // NOTE: To handle integer overflow,
    // read https://rust-book.cs.brown.edu/ch03-02-data-types.html#integer-overflow

    // Floating-point
    let _ = 2.0; // f64
    let _: f32 = 3.0; // f32

    // Numeric operation
    // +, -, *, /, and %

    // Boolean
    let _ = true;
    let _: bool = false;

    // Character
    let _ = 'z';
    let _: char = 'ℤ';
    let _ = '😻';

    println!("");
}

fn compound_types() {
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, _, _) = tup; // Destruct
    let _ = tup.2;
    // Tuple: modifying individual elements
    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
    // NOTE: The tuple without any values has a special name, 'unit'.

    // Array
    let _ = [1, 2, 3, 4, 5];
    let _: [i32; 5] = [1, 2, 3, 4, 5];
    let _ = [3; 5];
}
