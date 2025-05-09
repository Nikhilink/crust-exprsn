use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1
    {
        panic!("No expression provided...");
    }
    else {
        println!("provided argument {}", args[1]);
    }
}
