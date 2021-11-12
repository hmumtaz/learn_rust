fn main() {
    // let mut temperature = String::new();

    converter("c", 0.0)
}

fn f_to_c(temp: f64) {
    let new_temperature = (5.0 * (temp - 32.0)) / 9.0;
    println!("{} Fahrenheit is {:.2} in Celsius", temp, new_temperature)
}

fn c_to_f(temp: f64) {
    let new_temperature = ((temp * 9.0) / 5.0) + 32.0;
    println!("{} Celsius is {:.2} in Fahrenheit", temp, new_temperature)
}

fn converter(f_or_c: &str, init_temp: f64) {
    if f_or_c == "F" || f_or_c == "f" {
        f_to_c(init_temp);
    } else if f_or_c == "C" || f_or_c == "c" {
        c_to_f(init_temp);
    } else {
        println!("Invalid Input. This program can only convert temperatures from Fahrenheit to Celsius or Celsius to Fahrenheit.")
    }
}