#[macro_use]
extern crate time_test;
mod sqlparser;
fn main() {
    assert![String::from("INSERT INTO A ").starts_with("INSERT INTO")];
    println!("Hello, world!");
    println!("Assert passed");

}
