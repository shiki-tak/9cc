use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid number of arguments, args={}", args.len());
        return
    }

    print!(".intel_syntax noprefix\n");
    print!(".global _main\n");
    print!("_main:\n");
    print!("  mov rax, {}\n", args[1]);
    print!("  ret\n");
}
