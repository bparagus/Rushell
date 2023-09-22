use rushell_improved::lexer::lex;
use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line) => {
                let tokens = lex(&line);
                println!("{tokens:#?}");
            }
            Err(e) => {
                match e.to_string().as_str() {
                    "Interrupted" => continue,
                    &_ => {
                        println!("{}", e);
                        std::process::exit(1)
                    }
                }
            }
        }
    }
}
