use std::io;

enum Temp {
    F(f64),
    C(f64),
}

// This function does not need ownership
fn convert_temperature(temp: &Temp) -> f64 {
    match temp {
        &Temp::F(degrees) => (degrees - 32.0) / 1.8,    // convert to Celsius
        &Temp::C(degrees) => (degrees * 1.8) + 32.0,    // convert to Fahrenheit
    }
}

fn print_temperature(temp: &Temp) {
    match temp {
        &Temp::F(degrees) => println!("{}F = {:.0}C", degrees, convert_temperature(temp)),
        &Temp::C(degrees) => println!("{}C = {:.0}F", degrees, convert_temperature(temp)),
    }
}

fn main() {
    println!("Temperature Convertor - Enter the temperature (e.g. 25C or 40F):");
    let mut input = String::new();
    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

    let input = input.trim();
    let (temp, scale) = input.split_at(input.len() - 1);

    let temp: f64 = match temp.parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let temp: Temp = match scale {
        "C" => Temp::C(temp),
        "F" => Temp::F(temp),
        &_ => Temp::C(0.0),
    };

    print_temperature(&temp);
}