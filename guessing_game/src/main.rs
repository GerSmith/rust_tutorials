/*
    Программа генерирует случайное целое число от 1 до 100.
    Затем она предлагает игроку ввести и отгадать число.
    Если оно больше или меньше предложенного игроком,
    то программа сообщит об этом. Если игрок угадал число,
    то программа выведет поздравление и завершится
*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Игра \"Угадай число!\"");
    // Наше секретное число
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);
    // Счётчик попыток угадать
    let mut attempts: u32 = 0;
    // Основной цикл игры
    loop {
        println!("Пожалуйста введите число от 1 до 100:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Ошибка чтения!");
        // Обработка преобразования строки ввода
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                // Логическая проверка на вхождение в диапазон
                if num < 1 || num > 100 {
                    println!("Необходмо ввести число от 1 до 100, а не {}!", num);
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Необходмо ввести число от 1 до 100! Попробуй ещё раз!");
                continue;
            }
        };
        // Тестовый вывод
        // println!("Ваша догадка: {}", guess);
        // Если прошли все условия, увеличиваем счётчик попыток
        attempts += 1;
        // Шаблон сравнения
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Много"),
            Ordering::Less => println!("Мало"),
            Ordering::Equal => {
                println!("Молодец! Ты угадал!");
                println!("Тебе понадобилось {} попыток.", attempts);
                break;
            }
        }
        // Тестовый вывод
        // println!("Загаданное число: {}", secret_number);
    }
}
