extern crate clap;
use clap::{App, SubCommand, Arg, AppSettings};
mod temperature;
mod fib;
mod song;

fn main() {
    let app = App::new("Control Flow in Rust!")
       .version("1.0")
       .about("- Convert between Celsius and Farenheit \n- Generate fib sequences \n- Sing \"The Twelve Days of Christmas\"")
       .author("Jack Mordaunt")
       .subcommand(SubCommand::with_name("convert")
            .about("Convert between celsius and farenheit")
            .setting(AppSettings::AllowNegativeNumbers)
            .arg(Arg::with_name("from")
                .long("from")
                .short("f")
                .default_value("celsius")
                .help("Scale to convert from ('celsius', 'farenheit')"))
            .arg(Arg::with_name("value")
                .required(true)
                .number_of_values(1)))
        .subcommand(SubCommand::with_name("fib")
            .about("Generate the specified fibonacci number")
            .arg(Arg::with_name("nth")
            .required(true)))
        .subcommand(SubCommand::with_name("sing")
            .about("Sing the twelve days of christmas"));

    match app.get_matches().subcommand() {
        ("convert", Some(cmd)) => do_convert(cmd),
        ("fib", Some(cmd)) => do_fib(cmd),
        ("sing", Some(cmd)) => do_sing(cmd),
        ("", None) => println!("enter a subcommand"),
        _ => println!("unknown command"),
    }
}

fn do_convert(cmd: &clap::ArgMatches) {
    let value: f32 = match cmd.value_of("value").unwrap().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Value not a valid number: {}.", err);
            return
        }
    };
    let scale: &str = match cmd.value_of("from") {
        Some(scale) => scale,
        None => {
            println!("Tell me what scale to use (celsius or farenheit).");
            return 
        }
    };
    match scale {
        "celsius" => println!("Farenheit: {}", temperature::to_farenheit(value)),
        "farenheit" => println!("Celsius: {}", temperature::to_celsius(value)),
        _ => println!("Unknown scale {}, use celsius or farenheit.", scale),
    }
}

fn do_fib(cmd: &clap::ArgMatches) {
    let nth: u64 = match cmd.value_of("nth").unwrap().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Value is not a valid number: {}.", err);
            return 
        }
    };
    println!("{}", fib::calculate(nth));
}

fn do_sing(_cmd: &clap::ArgMatches) {
    print!("\n{}\n", song::sing());
}