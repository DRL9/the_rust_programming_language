use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

fn main() {
    Pancakes::hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;
