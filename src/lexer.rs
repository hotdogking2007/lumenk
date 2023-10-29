
#[derive(Debug,PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Dot,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    Assign,
    Semicolon,
    ReturnType,

    Bang,
    Assert,

    If,
    Else,
    Func,
    Var,
    Const,
    Class,

    Number,
    String,
    Identifier,
    Sign,

    EOF,
    Error
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tt: TokenType,
    pub value: String,
    pub row: usize,
    pub column: usize,
}
impl Token {
    pub fn new(tt: TokenType, v: String, row:usize, column: usize) -> Token {
        Token {
            tt,
            value: v,
            row,
            column
        }
    }
}

pub struct Lexer {
    pub token_list: Vec<Token>,
    text: String,
    line: usize,
    row: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            token_list: Vec::new(),
            text: String::from(text)+&String::from("\0"),
            row:0,
            line:0
        }
    }
    pub fn run(&mut self) {
        let mut next: usize = 0;
        let source: Vec<char> = self.text.chars().collect();
        loop {
            self.row+=1;
            match source[next] {
                '(' => self.add_token(TokenType::LeftParen, "(".into()),
                ')' => self.add_token(TokenType::RightParen, ")".into()),
                '{' => self.add_token(TokenType::LeftBrace, "{".into()),
                '}' => self.add_token(TokenType::RightBrace, "}".into()),


                '.' => self.add_token(TokenType::Dot, ".".into()),
                ',' => self.add_token(TokenType::Comma, ",".into()),
                '+' => self.add_token(TokenType::Plus, "+".into()),
                '-' => {
                    if source[next+1] == '>' {
                        self.add_token(TokenType::ReturnType, "->".into());
                        next += 1;
                    } else {
                        self.add_token(TokenType::Minus, "-".into());
                    }
                    
                },
                '*' => self.add_token(TokenType::Star, "*".into()),
                '/' => self.add_token(TokenType::Slash, "/".into()),
                '%' => self.add_token(TokenType::Mod, "%".into()),
                '=' => {
                    if source[next+1] == '=' {
                        self.add_token(TokenType::Assert, "==".into());
                        next += 1;
                    } else {
                        self.add_token(TokenType::Assign, "=".into());
                    }
                    
                },
                ';' => self.add_token(TokenType::Semicolon, ";".into()),
                


                '\n' => self.line+=1,
                '"' => {
                    let n = self.string(source.clone(), &mut next);
                    next+=1;
                    self.add_token(TokenType::String, n);
                },
                _ => {
                    if source[next].is_numeric() {
                        let n = self.number(source.clone(), &mut next);
                        next -= 1;
                        self.add_token(TokenType::Number, n);
                    } else if source[next].is_ascii_whitespace() {
                    } else if source[next].is_alphabetic(){
                        let n = self.identifier(source.clone(), &mut next);
                        match n.as_str() {
                            "if" => self.add_token(TokenType::If, "if".into()),
                            "else" => self.add_token(TokenType::Else, "else".into()),
                            "func" => self.add_token(TokenType::Func, "func".into()),
                            "var" => self.add_token(TokenType::Var, "var".into()),
                            "const" => self.add_token(TokenType::Const, "const".into()),
                            "class" => self.add_token(TokenType::Class, "class".into()),
                            _ => self.add_token(TokenType::Identifier, n),
                        }
                        
                    } else if source[next] == '\0' {
                        self.add_token(TokenType::EOF, "".to_owned());
                    } else {
                        self.add_token(TokenType::Error, format!("token error '{}'", source[next]).into());
                    }
                },
            }
            next+=1;
            if next >= source.len() {
                break;
            }
        }
    }
    pub fn add_token(&mut self, tt:TokenType, v: String) {
        self.token_list.push(
            Token {
                tt,
                value: v,
                row: self.row,
                column: self.line,
            }
        )
    }
    fn number(&mut self, source: Vec<char>, i: &mut usize) -> String {
        let start: usize = i.clone();
        let mut end: usize = i.clone();
        while source[*i].is_digit(10) {
            end += 1;
            *i += 1;
        }
        source[start..end].iter().collect()
    }
    fn string(&mut self, source: Vec<char>, i: &mut usize) -> String {
        let start: usize = i.clone()+1;
        let mut end: usize = i.clone() + 1;
        while source[end] != '\"' {
            end += 1;
            *i += 1;
            if source[end] == '\0'{
                break;
            }
        }
        source[start..end].iter().collect()
    }
    fn identifier(&mut self, source: Vec<char>, i: &mut usize) -> String {
        let start: usize = i.clone();
        let mut end: usize = i.clone() + 1;
        while source[end].is_alphabetic() {
            end += 1;
            *i += 1;
            if source[end] == '\0'{
                break;
            }
        }
        source[start..end].iter().collect()
    }
}