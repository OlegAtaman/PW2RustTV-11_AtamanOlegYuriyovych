use std::io;

fn main() {
    let mut result: f64 = 0.0;
    let mut first_input = true;

    loop {
        if first_input {
            println!("Введіть число:");
            result = match try_read_number() {
                Some(num) => num,
                None => continue,
            };
            first_input = false;
        } else {
            println!("Введіть дію (+, -, *, /, 0 для обнулення):");
            let operator = read_operator();

            if operator == '0' {
                println!("Очищено!");
                first_input = true;
                continue;
            }

            println!("Введіть число:");
            let num = match try_read_number() {
                Some(num) => num,
                None => continue,
            };

            result = match operator {
                '+' => add(result, num),
                '-' => subtract(result, num),
                '*' => multiply(result, num),
                '/' => divide(result, num),
                _ => {
                    println!("Невідомий оператор!");
                    result
                }
            };

            println!("Результат: {}", result);
        }
    }
}

fn try_read_number() -> Option<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні");
    match input.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Введіть коректне число!");
            None
        }
    }
}

fn read_operator() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні");
    input.trim().chars().next().unwrap_or(' ')
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        println!("Помилка: ділення на нуль!");
        a
    }
}