use lumenk::lexer::Lexer;
use lumenk::parser::Parser;

fn main() {
    let mut l: Lexer = Lexer::new(r#"
var um = "um"
"#);
    l.run();
    let mut p: Parser = Parser::new(l.token_list.clone());
    let ss = p.parse().unwrap();
    println!("{:#?} {:#?}", l.token_list, ss);
}
