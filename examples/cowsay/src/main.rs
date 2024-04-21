#[allow(warnings)]
mod bindings;

use std::env::args;

fn main() {
    let arg = args().nth(1).unwrap_or("I don't know what to say".into());
    println!("< {} >", arg);
    println!(r"--------------
    \   ^__^
     \  (oo)\_______
        (__)\       )\/\
            ||----w |
            ||     ||");
}
