/*
    конвертер температур из единиц Фаренгейта в единицы Цельсия
*/

use std::io;

fn farenheit_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn _celsius_farenheit(c: f64) -> f64 {
    c * 5.0 / 9.0 + 32.0
}

fn celsius_kelvin(c: f64) -> f64 {
    c + 273.15
}

fn main() {
    println!("Конвертер температур из единиц Фаренгейта в единицы Цельсия и Кельвина!");
    println!("Введите значение в градусах Фаренгейта, для выхода Ctrl+C");

    loop {
        let mut temp_fahr = String::new();

        io::stdin()
            .read_line(&mut temp_fahr)
            .expect("Ошибка ввода!");

        let temp_fahr: f64 = match temp_fahr.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Необходмо ввести число! Попробуй ещё раз!");
                continue;
            }
        };

        println!(
            "{} °F в градусах Цельсиях: {:.2} °C. В Кельвинах: {:.2} °K",
            temp_fahr,
            farenheit_celsius(temp_fahr),
            celsius_kelvin(farenheit_celsius(temp_fahr))
        );
    }
}
