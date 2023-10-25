use crate::lexer::*;
use crate::ast::*;

pub struct Parser {
    pub token_list: Vec<Token>,
    pub cur: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { token_list: tokens, cur: 0 }
    }
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.token_list[self.cur+1].tt != TokenType::EOF {
            println!("{:?}",self.token_list[self.cur]);
            statements.push(self.stmt_parse());
            if self.cur > 99 {
                break;
            }
        }
        Ok(statements)
    }
    fn stmt_parse(&mut self) -> Statement {
        match self.token_list[self.cur].tt {
            TokenType::Var => self.var_parse(),
            _ => {self.cur += 1; Statement::None}
        }
    }
    fn expr_parse(&mut self) -> Binary {
        let l = Box::from(Expression::Value(self.token_list[self.cur].clone()));
        self.cur+=1;
        let op = self.token_list[self.cur].clone();
        self.cur+=1;
        let r = Box::from(Expression::Value(self.token_list[self.cur].clone()));
        Binary { l: l, r: r, op: op }
    }
    fn var_parse(&mut self) -> Statement {
        let head = self.token_list[self.cur].clone();
        self.cur+=1;
        if self.token_list[self.cur].tt == TokenType::Assign {
            self.cur += 1;
            let var_struct: Binary = self.expr_parse();
        }
        let var_struct:Binary = self.expr_parse();
        Statement::Var { kind: head, name: var_struct.l.as_ref()., init: var_struct.r }
    }
}