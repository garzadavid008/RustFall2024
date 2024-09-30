use std::io::{self, Write};
use std::fs::File;
use std::io::{BufReader, BufRead};

struct Car {
    brand: String,
    model: String,
    year: u32,
}

fn reading_from_console() -> Car{
    let mut buffer = String::new();

    print!("What is your cars brand name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let brand = buffer.trim().to_string();
    buffer.clear();

    print!("What is your cars model? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("What year is your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().unwrap();
    buffer.clear();

    Car {brand, model, year}
}

fn create_and_write_to_file(car: &Car) {
        let mut file = File::create("User_Info.txt").unwrap();
        writeln!(file, "Brand {}", car.brand).unwrap();
        writeln!(file, "Model {}", car.model).unwrap();
        writeln!(file, "Year {}", car.year).unwrap();
}

fn read_file_line_by_line() {
    let file = File::open("User_Info.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}


fn main(){
    let car = reading_from_console();
    create_and_write_to_file(&car);
    read_file_line_by_line();
}

