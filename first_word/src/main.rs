/*
    пример функции, входным параметром которой является строка,
    а выходным значением функции является первое слово,
    которое будет найдено в этой строке
*/

fn main() {
    let phrase_as_str = "Hello world!";
    let phrase_as_string = "World hello!";

    // тестовый вывод
    println!("{}", phrase_as_str);
    println!("{}", phrase_as_string);

    // тест функции
    println!("{}", first_word(phrase_as_str));
    println!("{}", first_word(phrase_as_string));
}

fn first_word(some_string: &str) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[..i];
        }
    }

    &some_string[..]
}
