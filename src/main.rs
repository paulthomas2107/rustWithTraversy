mod print;
mod types;
mod vars;

fn main() {
    println!("Hello, world!");
    print::run();
    vars::run();
    types::run();
}
