use std::io;

fn main() {
    println!("Temperature Conversion: F to C or C to F");
    
    loop {
        println!("Select the conversion direction.");
        println!("1. Celsius -> Fahrenheit");
        println!("2. Fahrenheit -> Celsius");
        
        let direct = input_type();

        if direct.trim().eq("1") {
            convert_to_fahrenheit();
    
        } else if direct.trim().eq("2") {
            convert_to_celsius();
    
        } else {
            break;
        }
    }
}

fn convert_to_celsius() {
    println!("Enter the temperature to be converted. (-100F ~ 100F)");
    
    let target = input_type();
    
    let target:f64 = target.trim().parse().expect("Error!");

    let convert = (target - 32.0) / 1.8;
    println!("Farenheit : {}°F -> Celsius : {}°C", target, convert);
}

fn convert_to_fahrenheit() {
    println!("Enter the temperature to be converted. (-100C ~ 100C)");

    let target = input_type();

    let target:f64 = target.trim().parse().expect("Error!");

    let convert = (target * 1.8) + 32.0;
    println!("Celsius : {}°C -> Farenheit : {}°F", target, convert);
}

fn input_type() -> String {
    let mut rtn = String::new();

    io::stdin().read_line(&mut rtn).expect("Error!");

    rtn
}