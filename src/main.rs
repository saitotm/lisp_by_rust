use std::io::Write;
use std::io;

fn rep(input: &String) {
    print(&eval(&read(input)))
}

fn read(input: &String) -> String {
    String::from(input)
}

fn eval(ast: &String) -> String {
    String::from(ast)
}

fn print(ast: &String) {
    print!("{}", ast);
}

fn main() {
    loop {
        let mut buf = String::new();

        print!("user> ");
        io::stdout().flush().unwrap(); // flushをしないと read_lineの後にuser>が表示される
        // stdout().flushを使うには std::io::Writeが必要

        match io::stdin().read_line(&mut buf) {
            Ok(n) => {
                if n > 1 {
                    rep(&buf);
                } else {
                    break;
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }
}