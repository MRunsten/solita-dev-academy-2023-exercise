mod database;
mod model;

use std::error::Error;

pub type BoxedError = Box<dyn Error>;

fn main() {
    println!("Hello, world!");
}
