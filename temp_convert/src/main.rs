use std::io::stdin;

fn main() {
    // let mut temperature = String::new();

    println!("Convert temperature");
    println!("Are you trying to convert from Fahrenheit or Celsius?");
    println!("Enter 'f' for Fahrenheit or 'c' for Celsius");
    loop {
        let mut f_or_c = String::new();
        stdin().read_line(&mut f_or_c).expect("Failed to read line");
        let f_or_c = f_or_c.trim();
        if f_or_c != "f" && f_or_c != "c" && f_or_c != "F" && f_or_c != "C" {
            println!("Invalid Input. This program can only convert temperatures from Fahrenheit to Celsius or Celsius to Fahrenheit.")
        } else {
            if f_or_c == "F" || f_or_c == "f" {
                println!("You have chosen to convert from Fahrenheit to Celsius");
                println!("Enter the temperature in Fahrenheit:");
                loop {
                    let mut temperature = String::new();
                    stdin()
                        .read_line(&mut temperature)
                        .expect("Failed to read line");
                    let temperature: f64 = match temperature.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please input a number");
                            continue;
                        }
                    };
                    let new_temperature = (5.0 * (temperature - 32.0)) / 9.0;
                    println!(
                        "{} Fahrenheit is {:.2} in Celsius",
                        temperature, new_temperature
                    );
                    break;
                }
            } else {
                println!("You have chosen to convert from Celsius to Fahrenheit");
                println!("Enter the temperature in Celsius:");
                loop {
                    let mut temperature = String::new();
                    stdin()
                        .read_line(&mut temperature)
                        .expect("Failed to read line");
                    let temperature: f64 = match temperature.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please input a number");
                            continue;
                        }
                    };
                    let new_temperature = ((temperature * 9.0) / 5.0) + 32.0;
                    println!(
                        "{} Celsius is {:.2} in Fahrenheit",
                        temperature, new_temperature
                    );
                    break;
                }
            }
            break;
        }
    }
}
