use std::rc::Rc;
use crate::lexer::Token;
use Token::*;
// use Node::*;

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
    Int(i32),
    Lambda { args: Vec<String>, body: Vec<Rc<Node>> },
    Call { proc: String, params: Vec<Rc<Expr>> },
    Cond { cond: Rc<Expr>, conseq: Rc<Expr>, alter: Rc<Expr> },
}

#[derive(Debug, Clone)]
pub struct Var {
    ident: String,
    offset: usize,
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
                Expr::Int(*number)
            },
            OpenPar => {
                self.pos += 1;

                let mut proc = self.parse_lambda();

                if self.expect("+") { proc = "plus".to_string(); }
                else if self.expect("*") { proc = "mul".to_string(); }

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

    fn parse_lambda(&mut self) -> String {
        match self.token_list[self.pos] {
            Ident(ref ident) => {
                self.pos += 1;
                ident.clone()
            },
            OpenPar => {
                self.consume("lambda");

                let mut args: Vec<String> = Vec::new();
                self.consume("(");
                while let Ident(ref ident) = self.token_list[self.pos] {
                    args.push((*ident).clone());
                    self.pos += 1;
                }
                self.consume(")");

                let mut body: Vec<Rc<Node>> = Vec::new();
                while let Node::Expr(expr) = self.parse_node() {
                    body.push(Rc::new(Node::Expr(expr)));
                }

                // Expr::Lambda { args, body }
                "0".to_string()
            },
            _ => { panic!(""); },
        }
    }

    fn expect(&mut self, name: &str) -> bool {
        match self.token_list[self.pos] {
            Ident(ref ident) if *ident == name => {
                self.pos += 1;
                true
            },
            _ => {
                false
            },
        }
    }

    fn consume(&mut self, name: &str) {
        match self.token_list[self.pos] {
            Ident(ref ident) if *ident == name => {
                self.pos += 1;
            },
            OpenPar if name == "(" => {
                self.pos += 1;
            },
            ClosePar if name == ")" => {
                self.pos += 1;
            },
            _ => {
                println!("{} is unexpected", name);
            }
        }
    }
}