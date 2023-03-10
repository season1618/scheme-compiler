use std::rc::Rc;
use crate::lexer::Token;
use Token::*;
// use Node::*;
// use Expr::*;

#[derive(Debug, Clone)]
pub enum Node {
    Defn(Defn),
    Expr(Expr),
    Lambda { args_num: usize, body: Vec<Node> },
}

#[derive(Debug, Clone)]
pub struct Defn {
    pub offset: usize,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(usize),
    Bool(bool),
    Int(i32),
    Proc(String),
    Call { proc: Rc<Expr>, params: Vec<Rc<Expr>> },
    Cond { cond: Rc<Expr>, conseq: Rc<Expr>, alter: Rc<Expr> },
}

#[derive(Debug, Clone)]
pub enum Var {
    Global(String),
    Local(usize),
}

#[derive(Debug, Clone)]
pub struct Lambda {
    args_num: usize,
    body: Vec<Node>,
}

pub fn parse(token_list: Vec<Token>) -> (Vec<Node>, Vec<Node>) {
    Parser::new(token_list).parse_program()
}

#[derive(Debug)]
struct Env {
    vec: Vec<(String, usize)>,
    offset: usize,
}

impl Env {
    fn new() -> Self {
        Env {
            vec: Vec::new(),
            offset: 0,
        }
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn push(&mut self, name: String) {
        self.offset += 8;
        self.vec.push((name, self.offset));
    }

    fn pop(&mut self) {
        self.offset -= 8;
        self.vec.pop();
    }

    fn find(&self, name: String) -> Option<usize> {
        for var in &self.vec {
            if var.0 == name {
                return Some(var.1);
            }
        }
        None
    }
}

struct Parser {
    token_list: Vec<Token>,
    proc_list: Vec<Lambda>,
    pos: usize,
    env: Env,
}

impl Parser {
    fn new(token_list: Vec<Token>) -> Self {
        Parser {
            token_list: token_list,
            proc_list: Vec::new(),
            pos: 0,
            env: Env::new(),
        }
    }

    fn parse_program(&mut self) -> (Vec<Node>, Vec<Node>) {
        let mut node_list: Vec<Node> = Vec::new();
        while self.pos < self.token_list.len() {
            node_list.push(self.parse_node());
        }

        let mut lambda_list: Vec<Node> = Vec::new();
        for i in 0..self.proc_list.len() {
            match self.proc_list[i] {
                Lambda { args_num, ref body } => {
                    lambda_list.push(Node::Lambda { args_num, body: body.clone() });
                }
            }
        }
        println!("{:?}", lambda_list);
        println!("{:?}", node_list);
        (lambda_list, node_list)
    }

    fn parse_node(&mut self) -> Node {
        if self.expect("(") {
            if self.expect("define") {
                return Node::Defn(self.parse_defn());
            } else {
                self.pos -= 1;
            }
        }
        return Node::Expr(self.parse_expr());
    }

    fn parse_defn(&mut self) -> Defn {
        match self.token_list[self.pos] {
            Ident(ref ident) => {
                self.pos += 1;
                let name = ident.clone();
                let expr = self.parse_expr();
                self.consume(")");

                self.env.push(name);
                Defn { offset: self.env.offset(), expr }
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
                
                if ident == "+" { return Expr::Proc("plus".to_string()); }
                if ident == "*" { return Expr::Proc("mul".to_string()); }

                for std_proc in ["cons", "car", "cdr"] {
                    if ident == std_proc {
                        return Expr::Proc(ident.clone());
                    }
                }
                
                match self.env.find(ident.clone()) {
                    Some(offset) => Expr::Var(offset),
                    None => {
                        println!("variable '{}' is undefined", ident);
                        panic!("");
                    }
                }
            },
            Bool(ref value) => {
                self.pos += 1;
                Expr::Bool(*value)
            },
            Number(ref number) => {
                self.pos += 1;
                Expr::Int(*number)
            },
            OpenPar => {
                self.pos += 1;

                if self.expect("lambda") {
                    let mut args_num = 0;
                    self.consume("(");
                    while let Ident(ref ident) = self.token_list[self.pos] {
                        self.env.push(ident.clone());
                        args_num += 1;
                        self.pos += 1;
                    }
                    self.consume(")");
    
                    let mut body: Vec<Node> = Vec::new();
                    while self.token_list[self.pos] != ClosePar {
                        body.push(self.parse_node());
                    }

                    self.consume(")");

                    for _ in 0..args_num {
                        self.env.pop();
                    }
    
                    // self.node_list.push(Node::Lambda { args_num, body });
                    self.proc_list.push(Lambda { args_num, body });
                    return Expr::Proc("_".to_string() + &self.proc_list.len().to_string());
                }

                let proc = Rc::new(self.parse_expr());
                let mut params: Vec<Rc<Expr>> = Vec::new();
                while self.token_list[self.pos] != ClosePar {
                    params.push(Rc::new(self.parse_expr()));
                }
                self.consume(")");

                return Expr::Call { proc, params };
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