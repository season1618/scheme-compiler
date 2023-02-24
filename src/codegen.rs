use std::fs::File;
use std::io::Write;
use crate::parser::*;

pub fn gen_asm(node_list: Vec<Node>, dest_path: String) {
    CodeGen::new(dest_path).gen_asm(node_list);
}

struct CodeGen {
    dest: File,
}

impl CodeGen {
    fn new(dest_path: String) -> Self {
        CodeGen {
            dest: File::create(dest_path).unwrap(),
        }
    }

    fn gen_asm(&mut self, node_list: Vec<Node>) {
        writeln!(self.dest, ".intel_syntax noprefix");
        writeln!(self.dest, ".global main");

        writeln!(self.dest, "plus:");
        writeln!(self.dest, "    mov rax, 0");
        writeln!(self.dest, "    add rax, QWORD PTR [rsp+8]");
        writeln!(self.dest, "    add rax, QWORD PTR [rsp+16]");
        writeln!(self.dest, "    ret");

        writeln!(self.dest, "main:");
        writeln!(self.dest, "    push rbp");
        writeln!(self.dest, "    mov rbp, rsp");

        for node in node_list {
            self.gen_node(node);
        }

        writeln!(self.dest, "    mov rsp, rbp");
        writeln!(self.dest, "    pop rbp");
        writeln!(self.dest, "    ret");
    }

    fn gen_node(&mut self, node: Node) {
        match node {
            Node::Defn(defn) => {},
            Node::Expr(expr) => self.gen_expr(expr),
        }
    }

    fn gen_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Int(val) => {
                writeln!(self.dest, "    push {}", val);
            },
            Expr::Call { proc, params } => {
                for param in params.into_iter().rev() {
                    self.gen_expr((*param).clone());
                }
                writeln!(self.dest, "    call {}", proc);
            },
            _ => {},
        }
    }
}