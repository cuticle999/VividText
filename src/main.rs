// cmd.rs
use clap::{Arg, Command};
use vividtext::colorful_text::{self, OutputFormat};

pub fn run_cli() {
    let matches = Command::new("colorful_text")
        .version("0.1.0")
        .about("Applies colorful gradients to text")
        .arg(Arg::new("text").required(true).index(1))
        .arg(Arg::new("format")
             .short('f')
             .long("format")
             .takes_value(true)
             .possible_values(["ansi", "eightbit"])
             .default_value("ansi"))
        .get_matches();

    let text = matches.value_of("text").unwrap();
    let format = match matches.value_of("format").unwrap() {
        "ansi" => OutputFormat::Ansi,
        "eightbit" => OutputFormat::EightBit,
        _ => unreachable!(),
    };

    let result = colorful_text::apply_gradient(text, format);
    println!("{}", result);
}
