use std::process::exit;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();

    let (verb, file) = if args.len() != 2 {
        eprintln!("usage: crypto-experiments (encrypt|decrypt) <filename>");
        exit(1)
    } else {
        (&args[0], &args[1])
    };

    println!("verb = {verb}");
    println!("file = {file}");
}
