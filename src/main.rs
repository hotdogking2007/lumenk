use lumenk::lexer::Lexer;
use lumenk::parser::Parser;

fn main() {
    let mut l: Lexer = Lexer::new(r#"
var um = -"um"
var a = *1
var c = 198+1-28*a
"#);
    l.run();
    let mut p: Parser = Parser::new(l.token_list.clone());
    let ss = p.parse().unwrap();
    println!("lexer token list: {:#?}\nparser ast: {:#?}", l.token_list, ss);
}
