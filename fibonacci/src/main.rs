/*
    Генератор чисел Фибоначчи
*/

use std::io;

fn fib(num: u32) -> u32 {
    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        fib(num - 1) + fib(num - 2)
    }
}

fn main() {
    println!("Генератор чисел Фибоначчи!");
    println!("Введите номер последовательности от 0 до ∞!");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Ошибка ввода!");

    let num: u32 = num.trim().parse().expect("Ошибка преобразования!");

    println!("Последовательность чисел Фибоначчи:");
    for n in 0..num + 1 {
        println!("n = {}, Fn = {}", n, fib(n));
    }
}
