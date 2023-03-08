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
    pub offset: usize,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Var(usize),
    Lambda { args: Vec<String>, body: Vec<Rc<Node>> },
    Call { proc: String, params: Vec<Rc<Expr>> },
    Cond { cond: Rc<Expr>, conseq: Rc<Expr>, alter: Rc<Expr> },
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub offset: usize,
}

pub fn parse(token_list: Vec<Token>) -> Vec<Node> {
    Parser::new(token_list).parse_program()
}

struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn next(&mut self) -> usize {
        self.count += 8;
        self.count
    }
}

#[derive(Debug)]
struct Env {
    vec: Vec<Var>,
}

impl Env {
    fn new() -> Self {
        Env { vec: Vec::new() }
    }

    fn push(&mut self, name: String, offset: usize) {
        self.vec.push(Var { name, offset });
    }

    fn find(&self, name: String) -> Option<usize> {
        for var in &self.vec {
            if var.name == name {
                return Some(var.offset);
            }
        }
        None
    }
}

struct Parser {
    token_list: Vec<Token>,
    pos: usize,
    counter: Counter,
    env: Env,
}

impl Parser {
    fn new(token_list: Vec<Token>) -> Self {
        Parser {
            token_list: token_list,
            pos: 0,
            counter: Counter::new(),
            env: Env::new(),
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
        if self.expect("(") {
            if self.expect("define") {
                return Node::Defn(self.parse_defn());
            } else {
                self.pos -= 1;
                return Node::Expr(self.parse_expr());
            }
        }
        panic!("");
    }

    fn parse_defn(&mut self) -> Defn {
        match self.token_list[self.pos] {
            Ident(ref ident) => {
                self.pos += 1;
                let name = ident.clone();
                let offset = self.counter.next();
                let expr = self.parse_expr();
                self.consume(")");

                self.env.push(name, offset);
                Defn { offset, expr }
            },
            _ => {
                panic!("not identifier.");
            },
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match self.token_list[self.pos] {
            Ident(ref ident) => {
                self.pos += 1;
                println!("{:?}", self.env);
                match self.env.find(ident.clone()) {
                    Some(offset) => {
                        Expr::Var(offset)
                    },
                    None => {
                        println!("varibale '{}' is undefined", ident);
                        panic!("");
                    }
                }
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
            OpenPar if name == "(" => {
                self.pos += 1;
                true
            },
            ClosePar if name == ")" => {
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