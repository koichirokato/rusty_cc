use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    let input = &args[1];
    let value: i32 = match input.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("引数が数値ではありません: {}", input);
            process::exit(1);
        }
    };

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", value);
    println!("  ret");
}
