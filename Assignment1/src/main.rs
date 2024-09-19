
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0 
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (9.0 / 5.0) * c + 32.0 
}

fn main() {
    const FREEZING_POINT: i32 = 32;

    let mut f_temp: f64 = 93.0;

    println!("Fahrenheit to Celsius: {}", fahrenheit_to_celsius(f_temp));

    let mut counter = 0;
    loop {
        f_temp += 1.0;
        println!("{}°F is {:.2}°C", f_temp, fahrenheit_to_celsius(f_temp));
        counter += 1;
        if counter == 5 {
            break;
        }
    }
}