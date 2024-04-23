#[allow(warnings)]
mod bindings;

use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let message = if args.len() <= 1 {
        "I don't know what to say".into()
    } else {
        let args = &args[1..];
        let args = args.join(" ");
        args
    };

    let width = message.len() + 2;
    let underline = "-".repeat(width);

    println!("< {} >", message);
    println!(r" {}", underline);
    println!(
        r"    \   ^__^
     \  (oo)\_______
        (__)\       )\/\
            ||----w |
            ||     ||"
    );
}
