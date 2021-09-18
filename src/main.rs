use std::io;

fn main() {
    loop {
        // Main Menu
        println!("Please select a conversion type:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        
        let mut menu_selection = String::new();

        io::stdin()
            .read_line(&mut menu_selection)
            .expect("Failed to read line");

        let menu_selection = menu_selection.trim();
        let menu_selection = match menu_selection {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("Please input 1 or 2");
                continue;
            }
        };

        // Temperature input
        println!("Please input a temperature (float):");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid temperature");
                continue;
            }
        };
        
        // Temperature conversion and return the result
        if menu_selection == 1 {
            let converted_temperature = far2cel(temperature);
            println!("{} Fahrenheit = {} Celcius", temperature, converted_temperature)

        } else if menu_selection == 2 {
            let converted_temperature = cel2far(temperature);
            println!("{} Celcius = {} Fahrenheit", temperature, converted_temperature)
        };
        break;
    };
}

// convert from Fahrenheit to Celsius
fn far2cel(temperature: f64) -> f64 {
    (temperature - 32.) / 1.8
}
// convert from Celsius to Fahrenheit
fn cel2far(temperature: f64) -> f64 {
    temperature * 1.8 + 32.
}