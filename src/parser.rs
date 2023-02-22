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

#[derive(Debug, Clone)]
pub enum Expr {
    Var(Var),
    Object(Object),
    Call { proc: String, params: Vec<Rc<Expr>> },
    Cond { cond: Rc<Expr>, conseq: Rc<Expr>, alter: Rc<Expr> },
}

#[derive(Debug, Clone)]
pub struct Var {
    ident: String,
    offset: usize,
}

#[derive(Debug, Clone)]
pub enum Object {
    Int(i32),
    Lambda(Lambda),
}

#[derive(Debug, Clone)]
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
            node_list.push(self.parse_node());
        }
        println!("{:?}", node_list);
        node_list
    }

    fn parse_node(&mut self) -> Node {
        match self.token_list[self.pos] {
            OpenPar => {
                if self.expect("define") {
                    panic!("");
                }
            },
            _ => {},
        }

        Node::Expr(self.parse_expr())
    }

    fn parse_expr(&mut self) -> Expr {
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
                Expr::Object(Int(*number))
            },
            OpenPar => {
                self.pos += 1;

                let mut proc = "1".to_string();

                if self.expect("+") { proc = "plus".to_string(); }

                let mut params: Vec<Rc<Expr>> = Vec::new();
                while self.token_list[self.pos] != ClosePar {
                    params.push(Rc::new(self.parse_expr()));
                }
                self.pos += 1;

                Expr::Call { proc, params }
            },
            ClosePar => {
                panic!("too much ')'");
            },
            Period => {
                panic!("'.' is invalid");
            },
        }
    }

    fn expect(&mut self, name: &str) -> bool {
        match self.token_list[self.pos] {
            Ident(ref ident) if *ident == name => {
                self.pos += 1;
                return true;
            },
            _ => {},
        }
        return false;
    }
}