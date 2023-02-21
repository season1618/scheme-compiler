use std::rc::Rc;
use crate::lexer::Token;
use Token::*;
// use Node::*;
use Object::*;

#[derive(Debug)]
pub enum Node {
    Defn(Defn),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Defn {
    var: String,
    expr: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Var(Var),
    Object(Object),
    Call { proc: Lambda, params: Vec<Rc<Expr>> },
    Cond { cond: Rc<Expr>, conseq: Rc<Expr>, alter: Rc<Expr> },
}

#[derive(Debug)]
pub struct Var {
    ident: String,
    offset: usize,
}

#[derive(Debug)]
pub enum Object {
    Int(i32),
    Lambda(Lambda),
}

#[derive(Debug)]
pub struct Lambda {
    args: Vec<String>,
    body: Vec<Rc<Node>>,
}

pub fn parse(token_list: Vec<Token>) -> Vec<Node> {
    Parser::new(token_list).parse_program()
}

struct Parser {
    token_list: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(token_list: Vec<Token>) -> Self {
        Parser {
            token_list: token_list,
            pos: 0,
        }
    }

    fn parse_program(&mut self) -> Vec<Node> {
        let mut node_list: Vec<Node> = Vec::new();
        while self.pos < self.token_list.len() {
            node_list.push(self.parse_defn_or_expr());
        }
        println!("{:?}", node_list);
        node_list
    }

    fn parse_defn_or_expr(&mut self) -> Node {
        match self.token_list[self.pos] {
            Ident(ref ident) => {
                self.pos += 1;
                panic!("not found");
            },
            Bool(ref value) => {
                self.pos += 1;
                panic!("  ");
            },
            Number(ref number) => {
                self.pos += 1;
                Node::Expr(Expr::Object(Int(*number)))
            },
            OpenPar => {
                self.pos += 1;
                panic!("");
                // Node::Expr(self.expr())
                
            },
            ClosePar => {
                panic!("too much ')'");
            },
            Period => {
                panic!("'.' is invalid");
            },
        }
    }
}