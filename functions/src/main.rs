/*
    Работа с функциями
*/

fn main() {
    println!("Hello, world!");

    // another_func();

    another_func(5);
    print_label_measurment(71, 'h');

    let x = plus_one(5);
    println!("plus_one func working.... It is = {}", x);

    let _y = 6; // this is operator

    let i = {
        // this is expression
        let j = 5.0;
        j + 1.3333
    };
    println!("The value of i is: {}", i)
}

fn another_func(x: i32) {
    println!("Hello one $_$ from another function");
    println!("You give me {}! Thanks!", x);
}

fn print_label_measurment(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
