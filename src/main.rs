
use hello_macro_lib::HelloMacro;
use hello_macro_derive::HelloMacro;

fn main() {
    println!("Hello, world!");
    Pancakes::hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;




