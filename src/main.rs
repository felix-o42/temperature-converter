use std::io;

fn main() {
    loop {
        println!("=====================");
        println!("Temperature Converter");
        println!("=====================");
        println!();

        println!("Convert Fahrenheit to Celsius and reverse!");

        // let the user decide whether to convert from celsius to fahrenheit or reverse
        let choice = get_choice();

        // converting from fahrenheit to celsius
        if choice == 1 {
            let temp_fahrenheit = get_temp_fahrenheit();
            let temp_celsius = convert_fahrenheit_to_celsius(temp_fahrenheit);

            println!();
            print!("{temp_fahrenheit}°F = {temp_celsius}°C");
            println!();

        }
        // converting celsius to fahrenheit
        else if choice == 2 {
            let temp_celsius = get_temp_celsius();
            let temp_fahrenheit = convert_celsius_to_fahrenheit(temp_celsius);

            println!();
            print!("{temp_celsius}°C = {temp_fahrenheit}°F");
            println!();
        }
    }
}

fn get_choice() -> u8 {
    loop {
        println!();
        println!("What would you like to convert?");
        println!("1. Fahrenheit -> Celsius");
        println!("2. Celsius -> Fahrenheit");
        println!();

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice < 1 || choice > 2 {
            continue;
        }

        return choice;
    }
}

fn get_temp_celsius() -> f64 {
    loop {
        println!();
        println!("Enter the temperature in Celsius");
        println!();

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // -273.150°C equals zero kelvin and is the lowest possible temperature
        if temp < -273.150 {
            continue;
        }

        return temp;
    }
}


fn get_temp_fahrenheit() -> f64 {
    loop {
        println!();
        println!("Enter the temperature in Fahrenheit");
        println!();

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // −459,670°F equals zero kelvin and is the lowest possible temperature
        if temp < -459.670 {
            continue;
        }

        return temp;
    }
}

fn convert_fahrenheit_to_celsius(temp_fahrenheit: f64) -> f64 {
    return (temp_fahrenheit - 32.0) / 1.8;
}

fn convert_celsius_to_fahrenheit(temp_celsius: f64) -> f64 {
    return temp_celsius * 1.8 + 32.0;
}