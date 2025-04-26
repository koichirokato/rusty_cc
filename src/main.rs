use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    let mut p = args[1].as_str();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // 最初の数値を取得
    let (num, rest) = parse_number(p);
    println!("  mov rax, {}", num);
    p = rest;

    while !p.is_empty() {
        let bytes = p.as_bytes();
        match bytes[0] as char {
            '+' => {
                p = &p[1..];
                let (num, rest) = parse_number(p);
                println!("  add rax, {}", num);
                p = rest;
            }
            '-' => {
                p = &p[1..];
                let (num, rest) = parse_number(p);
                println!("  sub rax, {}", num);
                p = rest;
            }
            c => {
                eprintln!("予期しない文字です: '{}'", c);
                process::exit(1);
            }
        }
    }

    println!("  ret");
}

fn parse_number(p: &str) -> (i64, &str) {
    let p = p.trim_start();
    let mut end = 0;

    for (i, c) in p.char_indices() {
        if !c.is_digit(10) {
            end = i;
            break;
        }
    }

    if end == 0 && !p.is_empty() && p.chars().all(|c| c.is_digit(10)) {
        end = p.len();
    }

    let (number_str, rest) = p.split_at(end);

    match number_str.parse::<i64>() {
        Ok(n) => (n, rest),
        Err(_) => {
            eprintln!("数値のパースに失敗しました: '{}'", number_str);
            process::exit(1);
        }
    }
}

