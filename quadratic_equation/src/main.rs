/*
    Решаем квадратное уравнение
    ax^2 + bx + x = 0
    a - первый коэффициент
    b - второй коэффициент
    c - свободный коэффициент
    Дискриминант D = b^2 - 4*a*c
*/

use std::io;

fn main() {
    println!("Решение квадратного уравнения!");
    println!("ax^2 + bx + x = 0");
    println!("Для выхода нажмите Ctrl+C");

    loop {
        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();

        println!("Введите а: ");
        match io::stdin().read_line(&mut a) {
            Ok(_) => {}
            Err(e) => {
                println!("Ошибка чтения: {}", e)
            }
        };
        let a: f64 = a.trim().parse().expect("Ошибка преобразования!");

        println!("Введите b: ");
        match io::stdin().read_line(&mut b) {
            Ok(_) => {}
            Err(e) => {
                println!("Ошибка чтения:  {}", e)
            }
        };
        let b: f64 = b.trim().parse().expect("Ошибка преобразования!");

        println!("Введите c: ");
        match io::stdin().read_line(&mut c) {
            Ok(_) => {}
            Err(e) => {
                println!("Ошибка чтения: {}", e)
            }
        };
        let c: f64 = c.trim().parse().expect("Ошибка преобразования!");

        // тестовый вывод
        println!("---------------------");
        println!("{}x^2 + {}x + {} = 0", a, b, c);
        println!("a = {}, b = {}, c = {}", a, b, c);

        let d: f64 = b.powi(2) - 4.0 * a * c;
        // тестовый вывод
        println!("Дискриминант, D = {}", d);

        if d > 0.0 {
            let x1 = (-b + d.sqrt()) / (2.0 * a);
            let x2 = (-b - d.sqrt()) / (2.0 * a);
            println!("Уравнение решено, два корня:");
            println!("{}, {}", x1, x2);
        } else if d == 0.0 {
            let x = -b / (2.0 * a);
            println!("Уравнение решено, один корень:");
            println!("{}", x);
        } else {
            println!("Корней не существует!");
        }
    }
}
