use std::io;
use std::io::Write;
use std::str::FromStr;

struct ParseModeError {}

enum Mode {
    F2C,
    C2F,
}
impl FromStr for Mode {
    type Err = ParseModeError;

    fn from_str(s: &str) -> Result<Mode, ParseModeError> {
        match s {
            "1" => Ok(Mode::F2C),
            "2" => Ok(Mode::C2F),
            _ => Err(ParseModeError {}),
        }
    }
}

fn main() {
    println!("Welcome to the temperature converter!");

    println!("Pick a conversion:");
    println!("[1] Fahrenheit to Celsius");
    println!("[2] Celsius to Fahrenheit");

    let choice: Mode = read_value_from_input(
        "> ",
        "Please enter a valid choice (0 or 1)!"
    );

    match choice {
        Mode::F2C => {
            let temperature: f64 = read_value_from_input(
                "Enter the temperature to convert: ",
                "Please enter a valid floating point variable!"
            );

            println!(
                "{:.2} °F is {:.2} °C.",
                temperature,
                (temperature - 32f64) * 5f64 / 9f64
            );
        },
        Mode::C2F => {
            let temperature: f64 = read_value_from_input(
                "Enter the temperature to convert: ",
                "Please enter a valid floating point variable!"
            );

            println!(
                "{:.2} °C is {:.2} °F.",
                temperature,
                temperature * 9f64 / 5f64 + 32f64
            );
        },
    }
}

fn read_value_from_input<T: FromStr>(prompt: &str, error_message: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush()
            .expect("Unable to flush STDOUT!");

        let mut input_value = String::new();
        io::stdin().read_line(&mut input_value)
            .expect("Unable to read STDIN!");

        match input_value.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("{}", error_message),
        }
    }
}