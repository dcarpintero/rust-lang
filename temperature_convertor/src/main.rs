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
    println!("Temperature Convertor!");

    let temps = [
        Temp::F(-40.0), // -40
        Temp::F(0.0),   // -18
        Temp::C(-40.0), // -40
        Temp::C(0.0),   // 32
    ];

    for temp in temps.iter() {
        print_temperature(temp);
    }
}