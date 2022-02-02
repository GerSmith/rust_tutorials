/*
    Train with variables
*/

fn main() {
    // let x = 5; // we need mutable
    let mut x = 5;
    println!("This is x = {}.", x);
    x = 6;
    println!("Now we will up x, x = {}.", x);
    println!("---------");
    // constants
    const LIMITS: u32 = 123_456;
    println!("This is const: {}.", LIMITS);
    println!("---------");
    // shadow
    let y = 66;
    println!("The start value of y is {}.", y);
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {}.", y);
    }
    println!("Now we will up y, y = {}.", y);
    println!("---------");
    let spaces = "   ";
    println!("We need spaces: {}.", spaces);
    let spaces = spaces.len();
    println!("We count spaces: {}.", spaces);
    println!("---------");
    // annotations
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("This is parsing guess {}, but + 10: {}.", guess, guess + 10);
    let guess: i32 = "-42".parse().expect("Not a number!");
    println!("This is parsing guess {}, but + 10: {}.", guess, guess + 10);
    println!("---------");
    // scalar data types
    // integer
    println!("Suffix {}.", 57u8);
    println!("Dec {}.", 98_200);
    println!("Hex {0:x}, but HEX {0:X}.", 0xFF);
    println!("Oct {:o}.", 0o77);
    println!("Bin {:b}.", 0b1111_0000);
    println!("Byte {}.", b'A');
    println!(
        "unsigned integer u8 start from {} to {}.",
        0u8,
        2u32.pow(8) - 1
    );
    println!(
        "integer i8 start from {} to {}.",
        -2i32.pow(8 - 1),
        2i32.pow(8 - 1) - 1
    );
    // floating point
    let i = 2.0; // f64
    let j: f32 = 3.0; // f32
    println!("{} + {} = {}.", i, j, i + j);
    // operations
    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0
                          // remainder
    let _remainder = 43 % 5;
    println!("---------");
    // bool
    let _t = true;
    let _f: bool = false; // with explicit type annotation
    println!("---------");
    // char
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    println!("---------");
    // complex data types
    // tuple
    let tup = (500, 6.6, false, 33, true, 1.333333);
    println!("Tuple {:#?}", tup);
    println!(
        "1st = {}, 2nd = {}, 3rd = {}, 4th = {}, etc.",
        tup.0, tup.1, tup.2, tup.3
    ); // this is indexing
    let _five_hundred = tup.0; // this is indexing too
    let (q, w, e, r, t, y) = tup; // this is destructuring
    println!("qwerty is {}, {}, {}, {}, {}, {}", q, w, e, r, t, y);
    println!("---------");
    // array
    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
    let _b = [0; 10]; // short
    let _month = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("---------");
}
