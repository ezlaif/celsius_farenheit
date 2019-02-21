use std::io;
const CONVERSION_DELIMITER: f32 = 32.0;
const CONVERSION_RATE: f32 = 5.0/9.0;

fn main() {
    println!("Please enter dem Fahrenheits!");
    loop {
        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit)
            .expect("Failed to read dem Fahrenheits");
        let fahrenheit: f32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        let celsius = (fahrenheit - CONVERSION_DELIMITER) * CONVERSION_RATE;
        println!("{} °C", celsius);
    }
}
