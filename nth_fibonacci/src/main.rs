use std::io;

fn main() {
    loop {
        println!("Generate the nth Fibonacci number, please select n");


        let mut input_value = String::new();
        io::stdin().read_line(&mut input_value)
            .expect("Unable to read STDIN!");
        let input_value: i32 = match input_value.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };
        let nth: i32 = fib(input_value);

        println!("generated {nth}");

    }
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
