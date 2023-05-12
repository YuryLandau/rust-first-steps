use ferris_says::say;

// extern crate ferris_says;
use std::{
    fmt::Display,
    io::{stdout, BufWriter},
};
/// This line goes to doc for main function
fn main() {
    //! This line goes to doc
    //! Second doc line
    println!("{} days", 31);
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    Display!(make_string(927, "label"))
}
