use std::io;

enum Temp {
    F(f64),
    C(f64),
}

fn main() {
    let temp = input_temperature();
    convert_temperature(&temp);
}

fn input_temperature() -> Temp {
    println!("Temperature Convertor - Enter the temperature (e.g. 25C or 40F):");
    let mut input = String::new();
    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

    let (value, scale) = input.trim().split_at(input.trim().len() - 1);
    let value: f64 = value.parse().unwrap();

    let temp: Temp = match scale {
        "C" => Temp::C(value),
        "F" => Temp::F(value),
        &_ => panic!("Invalid temperature format!"),
    };
    
    return temp
}

fn convert_temperature(temp: &Temp) {
    match temp {
        &Temp::F(degrees) => println!("{}F = {:.0}C", degrees, convert_from_fahrenheit(degrees)),    // convert to Celsius
        &Temp::C(degrees) => println!("{}C = {:.0}F", degrees, convert_from_celsius(degrees)),       // convert to Fahrenheit
    }
}

fn convert_from_fahrenheit (degrees: f64) -> f64 {
    return (degrees - 32.0) / 1.8;
}

fn convert_from_celsius (degrees: f64) -> f64 {
    return (degrees * 1.8) + 32.0;
}