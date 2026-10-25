use rand::RngExt;
use std::{cmp::Ordering, io};

fn main() {
    println!("Игра: Угадай число");

    let secret_number = rand::rng().random_range(1..=100);
    // println!("Загаданное число: {secret_number}");
    loop {
        println!("Введите свое число:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Возникла ошибка при получении переменной");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Введенное число: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Меньше"),
            Ordering::Greater => println!("Больше"),
            Ordering::Equal => {
                println!("Ура, ты угадал");
                break;
            }
        }
    }
}
