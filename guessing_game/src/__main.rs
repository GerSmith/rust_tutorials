use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    // приветствие
    println!("Игра Угадай число!");
    println!("Я загадал целое число от 1 до 100. Тебе нужно его угадать, пробуй! ;)");

    // загадали число
    let secret_number = rand::thread_rng().gen_range(0..101);

    // запуск цикла, угадывай до посинения ;)))
    loop {
        print!("Пожалуйста, введите число: ");
        io::stdout().flush().expect("Ошибка!");
        // переменная
        let mut guess = String::new();
        // читаем ввод
        io::stdin().read_line(&mut guess).expect("Ошибка чтения!");
        // приводим типы
        let guess: u32 = guess.trim().parse().expect("Пожалуйста, введите число!");
        // сравнение
        if guess >= 1 && guess < 101 {
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Мало!"),
                Ordering::Greater => println!("Много!"),
                Ordering::Equal => {
                    println!("Попал!");
                    break;
                }
            }
        } else {
            println!("Нужно же ввести число от 1 до 100!");
        }
        // для тестовой отладки
        // println!("Вы ввели: {}", guess);
        // println!("Секретное число: {}", secret_number);
    }
}
