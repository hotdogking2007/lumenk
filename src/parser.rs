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
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements: Vec<Stmt> = Vec::new();
        while self.token_list[self.cur].tt != TokenType::EOF {
            println!("{:?}",self.token_list[self.cur]);
            statements.push(self.stmt_parse());
        }
        Ok(statements)
    }
    fn stmt_parse(&mut self) -> Stmt {
        match self.token_list[self.cur].tt {
            TokenType::Var => self.var_parse(),
            _ => {self.cur += 1; Stmt::None}
        }
    }
    fn infix_op_parse(&mut self) -> Option<BinOp> {
        match self.token_list[self.cur].tt {
            TokenType::Plus => Some(BinOp::Add),
            TokenType::Minus => Some(BinOp::Minus),
            TokenType::Slash => Some(BinOp::Div),
            TokenType::Mod => Some(BinOp::Mod),
            TokenType::Star => Some(BinOp::Multi),
            _ => None,
        }
    }
    fn prefix_op_parse(&mut self) -> Option<PreOp> {
        match self.token_list[self.cur].tt {
            TokenType::Bang => Some(PreOp::Not),
            TokenType::Minus => Some(PreOp::Minus),
            TokenType::Star => Some(PreOp::Defer),
            _ => None,
        }
    }
    fn expr_parse(&mut self) -> Expr {
        if let Some(p) = self.prefix_op_parse() {
            self.cur += 1;
            let r = match self.token_list[self.cur].tt {
                TokenType::Number => Value::Number(self.token_list[self.cur].value.clone()),
                TokenType::String => Value::String(self.token_list[self.cur].value.clone()),
                TokenType::Identifier => Value::Identifier(self.token_list[self.cur].value.clone()),
                _ => Value::None
            };
            return Expr::PrefixExpression(p, Box::from(Expr::Literal(r)));
        }
        let l = match self.token_list[self.cur].tt {
            TokenType::Number => Value::Number(self.token_list[self.cur].value.clone()),
            TokenType::String => Value::String(self.token_list[self.cur].value.clone()),
            TokenType::Identifier => Value::Identifier(self.token_list[self.cur].value.clone()),
            _ => Value::None
        };
        
        self.cur += 1;
        let op = self.infix_op_parse();
        match op.clone() {
            Some(op) => op,
            None => {
                return Expr::Literal(l);
            }
        };

        self.cur += 1;
        let r:Expr = self.expr_parse();
        Expr::InfixExpression { l: Box::from(Expr::Literal(l)), r: Box::from(r), op: op.unwrap() }
    }
    fn var_parse(&mut self) -> Stmt {
        self.cur+=1;
        let head = self.token_list[self.cur].clone();
        self.cur+=1;
        let mut var_struct:Expr = Expr::Literal(Value::None);
        if self.token_list[self.cur].tt == TokenType::Assign {
            self.cur += 1;
        }
        var_struct = self.expr_parse();
        println!("expr {:?}", var_struct);
        if self.token_list[self.cur].tt != TokenType::EOF {
            self.cur+=1;
        }
        Stmt::Var { kind: head.clone(), name: head.clone(), init: Box::from(var_struct) }
    }
}