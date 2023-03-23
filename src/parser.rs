use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::hash_set::Iter;

use crate::lexer::Token;
use Token::*;
// use Node::*;
// use Expr::*;
use Var::*;

#[derive(Debug, Clone)]
pub enum Node {
    Defn(Defn),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Defn {
    pub var: Rc<RefCell<Var>>,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(Rc<RefCell<Var>>),
    Bool(bool),
    Int(i32),
    Proc(String, FVs),
    Call { proc: Rc<Expr>, params: Vec<Expr> },
    If { test: Rc<Expr>, conseq: Rc<Expr>, alter: Rc<Expr> },
}

#[derive(Debug, Clone)]
pub enum Var {
    Global(String),
    Local(String, usize, bool),
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub free_vars: FVs,
    pub free_num: usize,
    pub args_num: usize,
    pub local_num: usize,
    pub body: Vec<Node>,
}

#[derive(Debug)]
pub struct Env {
    pub vec: Vec<Vec<(String, Rc<RefCell<Var>>)>>,
}

impl Env {
    fn new() -> Self {
        Env { vec: vec![Vec::new()] }
    }

    fn local_num(&self) -> usize {
        self.vec.last().unwrap().len()
    }

    pub fn globals(&self) -> &Vec<(String, Rc<RefCell<Var>>)> {
        &self.vec[0]
    }

    fn last(&self) -> Rc<RefCell<Var>> {
        self.vec.last().unwrap().last().unwrap().clone().1
    }

    fn push_frame(&mut self) {
        self.vec.push(Vec::new());
    }

    fn push_global(&mut self, name: String) {
        let name2 = name.clone();
        self.vec[0].push((name, Rc::new(RefCell::new(Var::Global(name2)))));
    }
    fn push_local(&mut self, name: String) {
        let offset = (self.vec.last().unwrap().len() + 1) * 8;
        self.vec.last_mut().unwrap().push((name.clone(), Rc::new(RefCell::new(Var::Local(name.clone(), offset, false)))));
    }

    fn pop_frame(&mut self) {
        self.vec.pop();
    }

    fn find(&mut self, name: String) -> Option<Rc<RefCell<Var>>> {
        for i in (0..self.vec.len()).rev() {
            let frame = &self.vec[i];
            for var in frame {
                if var.0 == name {
                    match *var.1.borrow_mut() {
                        Local(_, _, ref mut is_free) => {
                            *is_free = 0 < i && i < self.vec.len() - 1;
                        },
                        _ => {},
                    }
                    return Some(var.1.clone());
                }
            }
        }
        None
    }

    fn find_fv(&mut self, name: String) -> Option<(usize, String)> {
        for i in (0..self.vec.len()).rev() {
            let frame = &self.vec[i];
            for var in frame {
                if var.0 == name {
                    match *var.1.borrow() {
                        Local(_, offset, is_free) if is_free => {
                            return Some((offset, name.clone()));
                        },
                        _ => {},
                    }
                }
            }
        }
        None
    }

    fn is_free(&self, name: String) -> bool {
        for i in (0..self.vec.len()).rev() {
            let frame = &self.vec[i];
            for var in frame {
                if var.0 == name {
                    return 0 < i && i < self.vec.len() - 1;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct FVs {
    pub set: HashSet<(usize, String)>,
}

impl FVs {
    pub fn new() -> Self {
        FVs { set: HashSet::new() }
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn iter(&self) -> Iter<'_, (usize, String)> {
        self.set.iter()
    }

    fn insert(&mut self, fv: (usize, String)) {
        self.set.insert(fv);
    }

    pub fn offset(&self, name: String) -> usize {
        for (i, fv) in self.set.iter().enumerate() {
            if fv.1 == name {
                return 8 * (i + 1);
            }
        }
        panic!("{} is not found", name);
    }

    pub fn include(&self, name: String) -> bool {
        for (i, fv) in self.set.iter().enumerate() {
            if fv.1 == name {
                return true;
            }
        }
        false
    }
}

pub struct Parser {
    token_list: Vec<Token>,
    pos: usize,
    pub env: Env,
    pub proc_list: Vec<Lambda>,
    pub node_list: Vec<Node>,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        Parser {
            token_list: token_list,
            pos: 0,
            env: Env::new(),
            proc_list: Vec::new(),
            node_list: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) {
        while self.pos < self.token_list.len() {
            let node = self.parse_global_expr();
            self.node_list.push(node);
        }
        println!("{:?}", self.node_list);
    }

    fn parse_global_expr(&mut self) -> Node {
        if self.expect("(") {
            if self.expect("define") {
                return Node::Defn(self.parse_defn_global());
            } else {
                self.pos -= 1;
            }
        }
        return Node::Expr(self.parse_expr(&mut FVs::new()));
    }

    fn parse_local_expr(&mut self, fv: &mut FVs) -> Node {
        if self.expect("(") {
            if self.expect("define") {
                return Node::Defn(self.parse_defn_local(fv));
            } else {
                self.pos -= 1;
            }
        }
        return Node::Expr(self.parse_expr(fv));
    }

    fn parse_defn_global(&mut self) -> Defn {
        match self.token_list[self.pos] {
            Ident(ref ident) => {
                self.pos += 1;
                let name = ident.clone();
                self.env.push_global(name);
                let expr = self.parse_expr(&mut FVs::new());
                self.consume(")");

                Defn { var: self.env.last(), expr }
            },
            _ => {
                panic!("not identifier.");
            },
        }
    }

    fn parse_defn_local(&mut self, fv: &mut FVs) -> Defn {
        match self.token_list[self.pos] {
            Ident(ref ident) => {
                self.pos += 1;
                let name = ident.clone();
                self.env.push_local(name);
                let expr = self.parse_expr(fv);
                self.consume(")");

                Defn { var: self.env.last(), expr }
            },
            _ => {
                panic!("not identifier.");
            },
        }
    }

    fn parse_expr(&mut self, fv: &mut FVs) -> Expr {
        match self.token_list[self.pos] {
            Ident(ref ident) => {
                self.pos += 1;
                
                if ident == "=" { return Expr::Proc("equal".to_string(), FVs::new()); }
                if ident == "!=" { return Expr::Proc("neq".to_string(), FVs::new()); }
                if ident == "<" { return Expr::Proc("lth".to_string(), FVs::new()); }
                if ident == "<=" { return Expr::Proc("leq".to_string(), FVs::new()); }
                if ident == ">" { return Expr::Proc("gth".to_string(), FVs::new()); }
                if ident == ">=" { return Expr::Proc("geq".to_string(), FVs::new()); }
                if ident == "+" { return Expr::Proc("add".to_string(), FVs::new()); }
                if ident == "-" { return Expr::Proc("sub".to_string(), FVs::new()); }
                if ident == "*" { return Expr::Proc("mul".to_string(), FVs::new()); }
                if ident == "/" { return Expr::Proc("div".to_string(), FVs::new()); }

                for std_proc in ["cons", "car", "cdr", "rem"] {
                    if ident == std_proc {
                        return Expr::Proc(ident.clone(), FVs::new());
                    }
                }
                
                match self.env.find(ident.clone()) {
                    Some(var) => {
                        match *var.borrow() {
                            Local(_, _, is_free) if is_free => {
                                fv.insert(self.env.find_fv(ident.clone()).unwrap());
                            },
                            _ => {},
                        }
                        Expr::Var(var)
                    },
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
                    self.env.push_frame();

                    let mut args_num = 0;
                    self.consume("(");
                    while let Ident(ref ident) = self.token_list[self.pos] {
                        self.env.push_local(ident.clone());
                        args_num += 1;
                        self.pos += 1;
                    }
                    self.consume(")");
    
                    let mut body: Vec<Node> = Vec::new();
                    let mut next_fv = FVs::new();
                    while self.token_list[self.pos] != ClosePar {
                        body.push(self.parse_local_expr(&mut next_fv));
                    }

                    self.consume(")");

                    let free_num = next_fv.len();
                    let local_num = self.env.local_num();
                    self.env.pop_frame();

                    for (_, name) in next_fv.iter() {
                        if self.env.is_free(name.clone()) {
                            let free_var = self.env.find_fv(name.clone()).unwrap();
                            fv.insert(free_var.clone());
                        }
                    }
    
                    let id = self.proc_list.len();
                    self.proc_list.push(Lambda { free_vars: next_fv.clone(), free_num, args_num, local_num, body });
                    return Expr::Proc(format!("_{}", id), next_fv);
                }

                if self.expect("if") {
                    let test = Rc::new(self.parse_expr(fv));
                    let conseq = Rc::new(self.parse_expr(fv));
                    let alter = Rc::new(self.parse_expr(fv));
                    self.consume(")");
                    return Expr::If { test, conseq, alter };
                }

                let proc = Rc::new(self.parse_expr(fv));
                let mut params: Vec<Expr> = Vec::new();
                while self.token_list[self.pos] != ClosePar {
                    params.push(self.parse_expr(fv));
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