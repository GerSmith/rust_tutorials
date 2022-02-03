/*
    Циклы
*/

fn main() {
    println!("loops!");
    // endlessly
    /*
    loop {
        println!("again!");
    }
    */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    println!("---------");

    let mut number = 10;

    while number != 0 {
        println!("!{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!("---------");
    // this is not good idea!
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    println!("---------");
    // better
    for element in a {
        println!("the value is: {}", element);
    }
    println!("---------");
    // once more better
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    println!("---------");
}
