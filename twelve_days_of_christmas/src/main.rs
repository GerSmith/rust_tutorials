/*
    12 дней Рождества — английская народная песня
*/

use std::io::{self, Write};

fn main() {
    println!("The Twelve Days of Christmas!");
    println!("12 дней Рождества!");

    print!("Какой сегодня день Рождества [1..12]? ");
    io::stdout().flush().expect("");

    let mut day = String::new();

    io::stdin().read_line(&mut day).expect("Ошибка чтения!");

    let day: i32 = day.trim().parse().expect("Пожалуйста, введите число!");

    if day < 1 || day > 12 {
        println!("Необходмо ввести число от 1 до 12, а не {}!", day);
    } else {
        // тестовый вывод
        // println!("{}", day);

        let days = [
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth", "eleventh", "twelfth",
        ];

        let gifts = [
            "A Partridge in a Pear Tree",
            "Two Turtle Doves",
            "Three French Hens",
            "Four Calling Birds",
            "Five Golden Rings",
            "Six Geese a Laying",
            "Seven Swans a Swimming",
            "Eight Maids a Milking",
            "Nine Ladies Dancing",
            "Ten Lords a Leaping",
            "Eleven Pipers Piping",
            "12 Drummers Drumming",
        ];

        println!(
            "On the {} day of Christmas my true love gave to me:",
            days[(day - 1) as usize]
        );

        for d in (0..day as usize).rev() {
            println!("{}", gifts[d]);
        }
    }
}
