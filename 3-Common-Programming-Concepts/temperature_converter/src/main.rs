use std::io;

fn main() {
    println!("Celsius - Fahrenheit Converter");

    loop {
        println!("Enter initial temperature unit (C or F): ");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        if choice.trim() == "c" {
            println!("Enter temperature in Celsius: ");
            let mut temp_in_celsius = String::new();

            io::stdin()
                .read_line(&mut temp_in_celsius)
                .expect("Failed to read line");

            let temp_in_celsius: f64 = match temp_in_celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };


            let temp_in_fahrenheit = temp_in_celsius * 1.8 + 32.0;

            println!("The temperature in Fahrenheit is {temp_in_fahrenheit}");
        }

        else if choice.trim() == "f" {
            println!("Enter temperature in Fahrenheit: ");
            let mut temp_in_fahrenheit = String::new();

            io::stdin()
                .read_line(&mut temp_in_fahrenheit)
                .expect("Failed to read line");

            let temp_in_fahrenheit: f64 = match temp_in_fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };


            let temp_in_celsius = (temp_in_fahrenheit - 32.0) / 1.8;

            println!("The temperature in Celsius is {temp_in_celsius}");
        }

        else {
            break
        }
    }
}
