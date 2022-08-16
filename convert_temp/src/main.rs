use std::io;

fn main() {
    loop {
        println!("What would you like to convert: \n1) Fahrenheit to Celsius\n2) Celsius to Fahrenheit\n3) Quit");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 3 {
            println!("Quitting");
            break;
        }

        println!("What degree value would you like to convert?");

        let mut degree = String::new();

        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read input");

        let degree: f32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 1 {
            println!("Converting {degree} Fahrenheit to Celsius");
            let celsius = (degree - 32.00) * (0.5556);
            println!("Converted amount is {celsius} celsius\n");
        }

        if option == 2 {
            println!("Converting {degree} Celsius to Fahrenheit");
            let fahrenheit = (degree * 1.8) + 32.0;
            println!("Converted amount is {fahrenheit} celsius\n");
        }

    }
}
