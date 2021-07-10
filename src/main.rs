use clap::{crate_authors, crate_description, crate_name, App, Arg};


/// Get data & process it
/// This is technically one line, because I thought it would be fun.
///
/// 1) Processes the CLI args, and accepts everything passed as a vec of strings
/// 2) Iterates word-by-word
/// 3) Iterates each word by char
/// 4) Constructs an emoji for each char
/// 5) Collects all the emoji strings into words again
/// 6) Prints the result
fn main() {
    println!(
        "{}",
        App::new(crate_name!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                Arg::with_name("text")
                    .takes_value(true)
                    .multiple(true)
                    .help("Text to convert")
                    .required(true),
            )
            .get_matches()
            .values_of("text")
            .unwrap()
            .collect::<Vec<_>>()
            .iter()
            .map(|word| word
                .chars()
                .into_iter()
                .map(|char| match char.is_alphabetic() {
                    true => format!(":regional_indicator_{}:", char.to_lowercase()),
                    false => char.to_string(),
                })
                .collect::<Vec<String>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    )
}
