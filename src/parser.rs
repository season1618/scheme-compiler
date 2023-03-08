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
    Var(usize),
    Bool(bool),
    Int(i32),
    Proc(String),
    Lambda { args: Vec<String>, body: Vec<Rc<Node>> },
    Call { proc: String, params: Vec<Rc<Expr>> },
    Cond { cond: Rc<Expr>, conseq: Rc<Expr>, alter: Rc<Expr> },
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
    vec: Vec<(String, usize)>,
}

impl Env {
    fn new() -> Self {
        Env { vec: Vec::new() }
    }

    fn push(&mut self, name: String, offset: usize) {
        self.vec.push((name, offset));
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
    pos: usize,
    var_cnt: Counter,
    lambda_cnt: usize,
    env: Env,
}

impl Parser {
    fn new(token_list: Vec<Token>) -> Self {
        Parser {
            token_list: token_list,
            pos: 0,
            var_cnt: Counter::new(),
            lambda_cnt: 0,
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
                let offset = self.var_cnt.next();
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

                // match self.token_list[self.pos] {
                //     Ident(ref ident) => {
                //         self.pos += 1;
                        
                //     },
                //     OpenPar => {
                //         self.consume("lambda");
        
                //         let mut args: Vec<String> = Vec::new();
                //         self.consume("(");
                //         while let Ident(ref ident) = self.token_list[self.pos] {
                //             args.push(ident.clone());
                //             self.pos += 1;
                //         }
                //         self.consume(")");
        
                //         let mut body: Vec<Rc<Node>> = Vec::new();
                //         while let Node::Expr(expr) = self.parse_node() {
                //             body.push(Rc::new(Node::Expr(expr)));
                //         }
        
                //         // Expr::Lambda { args, body }
                //         self.lambda_cnt += 1;
                //         (self.lambda_cnt - 1).to_string()
                //     },
                //     _ => { panic!(""); },
                // }

                if let Expr::Proc(proc) = self.parse_expr() {
                    let mut params: Vec<Rc<Expr>> = Vec::new();
                    while self.token_list[self.pos] != ClosePar {
                        params.push(Rc::new(self.parse_expr()));
                    }
                    self.pos += 1;

                    return Expr::Call { proc, params };
                }

                panic!("");
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