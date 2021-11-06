/** Ths is about how to write procedural macro and run it
*/
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Hehehe;

fn main() {
    Pancakes::hello_macro();
    Hehehe::hello_macro();
}