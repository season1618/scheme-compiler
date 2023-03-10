use std::fs::File;
use std::io::Write;
use crate::parser::*;

pub fn gen_asm(lambda_list: Vec<Node>, node_list: Vec<Node>, dest_path: String) {
    CodeGen::new(dest_path).gen_asm(lambda_list, node_list);
}

struct CodeGen {
    dest: File,
    lambda_num: usize,
}

impl CodeGen {
    fn new(dest_path: String) -> Self {
        CodeGen {
            dest: File::create(dest_path).unwrap(),
            lambda_num: 0,
        }
    }

    fn gen_asm(&mut self, lambda_list: Vec<Node>, node_list: Vec<Node>) {
        writeln!(self.dest, ".intel_syntax noprefix").unwrap();
        writeln!(self.dest, ".global main").unwrap();

        writeln!(self.dest, "cons:").unwrap();
        writeln!(self.dest, "    mov rdi, 2").unwrap();
        writeln!(self.dest, "    mov rsi, 8").unwrap();
        writeln!(self.dest, "    call calloc").unwrap();
        writeln!(self.dest, "    mov rdi, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rsi, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    mov QWORD PTR [rax], rdi").unwrap();
        writeln!(self.dest, "    mov QWORD PTR [rax+8], rsi").unwrap();
        writeln!(self.dest, "    ret").unwrap();
        
        writeln!(self.dest, "car:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rax]").unwrap();
        writeln!(self.dest, "    ret").unwrap();
        
        writeln!(self.dest, "cdr:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rax+8]").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "plus:").unwrap();
        writeln!(self.dest, "    mov rax, 0").unwrap();
        writeln!(self.dest, "    add rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    add rax, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        for proc in lambda_list {
            self.gen_node(proc);
        }

        writeln!(self.dest, "main:").unwrap();
        writeln!(self.dest, "    push rbp").unwrap();
        writeln!(self.dest, "    mov rbp, rsp").unwrap();
        writeln!(self.dest, "    sub rsp, 200").unwrap();

        for node in node_list {
            self.gen_node(node);
        }

        writeln!(self.dest, "    mov rsp, rbp").unwrap();
        writeln!(self.dest, "    pop rbp").unwrap();
        writeln!(self.dest, "    ret").unwrap();
    }

    fn gen_node(&mut self, node: Node) {
        match node {
            Node::Defn(defn) => self.gen_defn(defn),
            Node::Expr(expr) => self.gen_expr(expr),
            Node::Lambda { args_num, body } => {
                self.lambda_num += 1;
                let id = self.lambda_num;
                writeln!(self.dest, "_{}:", id);
                writeln!(self.dest, "    push rbp").unwrap();
                writeln!(self.dest, "    mov rbp, rsp").unwrap();
                writeln!(self.dest, "    sub rsp, {}", 8 * args_num).unwrap();

                for i in 1..args_num + 1 {
                    writeln!(self.dest, "    mov rax, QWORD PTR [rbp+{}]", 8 * (i + 1)).unwrap();
                    writeln!(self.dest, "    mov QWORD PTR [rbp-{}], rax", 8 * i).unwrap();
                }

                for expr in body {
                    match expr {
                        Node::Defn(defn) => self.gen_defn(defn),
                        Node::Expr(expr) => self.gen_expr(expr),
                        _ => {},
                    }
                }

                writeln!(self.dest, "    mov rsp, rbp").unwrap();
                writeln!(self.dest, "    pop rbp").unwrap();
                writeln!(self.dest, "    ret").unwrap();
            },
        }
    }

    fn gen_defn(&mut self, defn: Defn) {
        self.gen_expr(defn.expr);
        writeln!(self.dest, "    pop rax").unwrap();
        writeln!(self.dest, "    mov QWORD PTR [rbp-{}], rax", defn.offset).unwrap();
    }

    fn gen_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Int(val) => {
                writeln!(self.dest, "    push {}", val).unwrap();
            },
            Expr::Proc(name) => {
                writeln!(self.dest, "    lea rax, [rip+{}]", name).unwrap();
                writeln!(self.dest, "    push rax").unwrap();
            },
            Expr::Var(offset) => {
                writeln!(self.dest, "    push QWORD PTR [rbp-{}]", offset).unwrap();
            },
            Expr::Call { proc, params } => {
                for param in params.into_iter().rev() {
                    self.gen_expr((*param).clone());
                }
                self.gen_expr((*proc).clone());
                writeln!(self.dest, "    pop rax").unwrap();
                writeln!(self.dest, "    call rax").unwrap();
                writeln!(self.dest, "    push rax").unwrap();
            },
            _ => {},
        }
    }
}