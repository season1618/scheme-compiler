use std::fs::File;
use std::io::Write;
use crate::parser::*;

pub fn gen_asm(parser: Parser, dest_path: String) {
    CodeGen::new(dest_path).gen_asm(parser);
}

struct CodeGen {
    dest: File,
    lambda_num: usize,
    if_num: usize,
    call_num: usize,
}

impl CodeGen {
    fn new(dest_path: String) -> Self {
        CodeGen {
            dest: File::create(dest_path).unwrap(),
            lambda_num: 0,
            if_num: 0,
            call_num: 0,
        }
    }

    fn gen_asm(&mut self, parser: Parser) {
        writeln!(self.dest, ".intel_syntax noprefix").unwrap();
        writeln!(self.dest, ".global main").unwrap();

        writeln!(self.dest, ".data").unwrap();

        for global in parser.env.globals() {
            writeln!(self.dest, "{}:", global.0).unwrap();
            writeln!(self.dest, "    .zero 8").unwrap();
        }

        writeln!(self.dest, ".text").unwrap();

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

        writeln!(self.dest, "equal:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rdi, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    cmp rax, rdi").unwrap();
        writeln!(self.dest, "    sete al").unwrap();
        writeln!(self.dest, "    movzb rax, al").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "neq:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rdi, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    cmp rax, rdi").unwrap();
        writeln!(self.dest, "    setne al").unwrap();
        writeln!(self.dest, "    movzb rax, al").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "lth:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rdi, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    cmp rax, rdi").unwrap();
        writeln!(self.dest, "    setl al").unwrap();
        writeln!(self.dest, "    movzb rax, al").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "leq:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rdi, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    cmp rax, rdi").unwrap();
        writeln!(self.dest, "    setle al").unwrap();
        writeln!(self.dest, "    movzb rax, al").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "gth:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rdi, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    cmp rax, rdi").unwrap();
        writeln!(self.dest, "    setg al").unwrap();
        writeln!(self.dest, "    movzb rax, al").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "geq:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    mov rdi, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    cmp rax, rdi").unwrap();
        writeln!(self.dest, "    setge al").unwrap();
        writeln!(self.dest, "    movzb rax, al").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "add:").unwrap();
        writeln!(self.dest, "    mov rax, 0").unwrap();
        writeln!(self.dest, "    add rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    add rax, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "sub:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    sub rax, QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "div:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    cqo").unwrap();
        writeln!(self.dest, "    idiv QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        writeln!(self.dest, "rem:").unwrap();
        writeln!(self.dest, "    mov rax, QWORD PTR [rsp+8]").unwrap();
        writeln!(self.dest, "    cqo").unwrap();
        writeln!(self.dest, "    idiv QWORD PTR [rsp+16]").unwrap();
        writeln!(self.dest, "    mov rax, rdi").unwrap();
        writeln!(self.dest, "    ret").unwrap();

        for proc in parser.proc_list {
            self.gen_proc(proc);
        }

        writeln!(self.dest, "main:").unwrap();
        writeln!(self.dest, "    push rbp").unwrap();
        writeln!(self.dest, "    mov rbp, rsp").unwrap();
        writeln!(self.dest, "    sub rsp, 200").unwrap();

        for node in parser.node_list {
            self.gen_node(node);
        }

        writeln!(self.dest, "    pop rax").unwrap();
        writeln!(self.dest, "    mov rsp, rbp").unwrap();
        writeln!(self.dest, "    pop rbp").unwrap();
        writeln!(self.dest, "    ret").unwrap();
    }

    fn gen_proc(&mut self, proc: Lambda) {
        let id = self.lambda_num;
        self.lambda_num += 1;
        writeln!(self.dest, "_{}:", id).unwrap();
        writeln!(self.dest, "    push rbp").unwrap();
        writeln!(self.dest, "    mov rbp, rsp").unwrap();
        writeln!(self.dest, "    sub rsp, {}", 8 * (proc.free_num + proc.local_num)).unwrap();

        for i in 1..proc.free_num + 1 {
            writeln!(self.dest, "    mov rax, QWORD PTR [rbp+{}]", 8 * (i + 1)).unwrap();
            writeln!(self.dest, "    mov QWORD PTR [rbp-{}], rax", 8 * i).unwrap();
        }
        for i in 1..proc.args_num + 1 {
            writeln!(self.dest, "    mov rax, QWORD PTR [rbp+{}]", 8 * (proc.free_num + i + 1)).unwrap();
            writeln!(self.dest, "    mov QWORD PTR [rbp-{}], rax", 8 * (proc.free_num + i)).unwrap();
        }

        for expr in proc.body {
            match expr {
                Node::Defn(defn) => self.gen_defn(defn, &proc.free_vars),
                Node::Expr(expr) => self.gen_expr(expr, &proc.free_vars),
            }
        }

        writeln!(self.dest, "    pop rax").unwrap();
        writeln!(self.dest, "    mov rsp, rbp").unwrap();
        writeln!(self.dest, "    pop rbp").unwrap();
        writeln!(self.dest, "    ret").unwrap();
    }

    fn gen_node(&mut self, node: Node) {
        match node {
            Node::Defn(defn) => self.gen_defn(defn, &FVs::new()),
            Node::Expr(expr) => self.gen_expr(expr, &FVs::new()),
        }
    }

    fn gen_defn(&mut self, defn: Defn, free_vars: &FVs) {
        let fv_num = free_vars.len();
        self.gen_expr(defn.expr, free_vars);
        
        match *defn.var.borrow() {
            Var::Global(ref name) => {
                writeln!(self.dest, "    pop rax").unwrap();
                writeln!(self.dest, "    mov [rip+{}], rax", name.clone()).unwrap();
            },
            Var::Free(_, offset) => {
                writeln!(self.dest, "    mov rdi, 1").unwrap();
                writeln!(self.dest, "    mov rsi, 8").unwrap();
                writeln!(self.dest, "    call calloc").unwrap();
                writeln!(self.dest, "    pop rdi").unwrap();
                writeln!(self.dest, "    mov QWORD PTR [rax], rdi").unwrap();
                writeln!(self.dest, "    mov QWORD PTR [rbp-{}], rax", 8 * fv_num + offset).unwrap();
            },
            Var::Local(_, offset) => {
                writeln!(self.dest, "    pop rax").unwrap();
                writeln!(self.dest, "    mov QWORD PTR [rbp-{}], rax", 8 * fv_num + offset).unwrap();
            },
        }
    }

    fn gen_expr(&mut self, expr: Expr, free_vars: &FVs) {
        match expr {
            Expr::Bool(val) => {
                writeln!(self.dest, "    push {}", if val { 1 } else { 0 });
            },
            Expr::Int(val) => {
                writeln!(self.dest, "    push {}", val).unwrap();
            },
            Expr::Proc(name, fvs) => {
                writeln!(self.dest, "    mov rdi, 2").unwrap();
                writeln!(self.dest, "    mov rsi, 8").unwrap();
                writeln!(self.dest, "    call calloc").unwrap();

                writeln!(self.dest, "    lea rdi, [rip+{}]", name).unwrap();
                writeln!(self.dest, "    mov [rax], rdi").unwrap();
                writeln!(self.dest, "    mov QWORD PTR [rax+8], 0").unwrap();

                for (offset, name) in fvs.iter() {
                    writeln!(self.dest, "    push rax").unwrap();

                    writeln!(self.dest, "    mov rdi, 2").unwrap();
                    writeln!(self.dest, "    mov rsi, 8").unwrap();
                    writeln!(self.dest, "    call calloc").unwrap();

                    if free_vars.include(name.clone()) {
                        writeln!(self.dest, "    mov rdi, [rbp-{}]", free_vars.offset(name.clone())).unwrap();
                    } else {
                        let fv_num = free_vars.len();
                        writeln!(self.dest, "    mov rdi, [rbp-{}]", fv_num + offset).unwrap();
                    }

                    // writeln!(self.dest, "    mov rdi, [rbp-{}]", /*offset*/  free_vars.offset(name.clone())).unwrap();
                    // writeln!(self.dest, "    mov rdi, [rdi]").unwrap();
                    writeln!(self.dest, "    pop rsi").unwrap();
                    writeln!(self.dest, "    mov [rax], rdi",).unwrap();
                    writeln!(self.dest, "    mov [rax+8], rsi").unwrap();
                }

                writeln!(self.dest, "    push rax").unwrap();
            },
            Expr::Var(var) => {
                match *var.borrow() {
                    Var::Global(ref name) => {
                        writeln!(self.dest, "    mov rax, [rip+{}]", name.clone()).unwrap();
                        writeln!(self.dest, "    push rax").unwrap();
                    },
                    Var::Free(ref name, _) => {
                        writeln!(self.dest, "    push QWORD PTR [rbp-{}]", free_vars.offset(name.clone())).unwrap();
                        // writeln!(self.dest, "    mov rax, QWORD PTR [rbp-{}]", free_vars.offset(name.clone())).unwrap();
                        // writeln!(self.dest, "    push [rax]").unwrap();
                    },
                    Var::Local(_, offset) => {
                        let fv_num = free_vars.len();
                        writeln!(self.dest, "    push QWORD PTR [rbp-{}]", 8 * fv_num + offset).unwrap();
                    },
                }
            },
            Expr::Call { proc, params } => {
                let params_num = params.len();
                for param in params.into_iter().rev() {
                    self.gen_expr(param, free_vars);
                }

                self.gen_expr((*proc).clone(), free_vars);
                writeln!(self.dest, "    pop rax").unwrap();

                let call_num = self.call_num;
                self.call_num += 1;

                writeln!(self.dest, "begin{}:", call_num).unwrap();
                writeln!(self.dest, "    cmp rax, 0").unwrap();
                writeln!(self.dest, "    je end{}", call_num).unwrap();

                writeln!(self.dest, "    push [rax]").unwrap(); //
                writeln!(self.dest, "    mov rax, [rax+8]").unwrap();
                writeln!(self.dest, "    jmp begin{}", call_num).unwrap();

                writeln!(self.dest, "end{}:", call_num).unwrap();
                
                writeln!(self.dest, "    pop rax").unwrap();
                writeln!(self.dest, "    call rax").unwrap();
                
                writeln!(self.dest, "    add rsp, {}", 8 * params_num).unwrap();
                writeln!(self.dest, "    push rax").unwrap();
            },
            Expr::If { test, conseq, alter } => {
                let label1 = self.if_num;
                let label2 = self.if_num + 1;
                self.if_num += 2;

                self.gen_expr((*test).clone(), free_vars);

                writeln!(self.dest, "    pop rax").unwrap();
                writeln!(self.dest, "    cmp rax, 0").unwrap();
                writeln!(self.dest, "    je .L{}", label1).unwrap();

                self.gen_expr((*conseq).clone(), free_vars);
                writeln!(self.dest, "    jmp .L{}", label2).unwrap();

                writeln!(self.dest, ".L{}:", label1).unwrap();

                self.gen_expr((*alter).clone(), free_vars);

                writeln!(self.dest, ".L{}:", label2).unwrap();
            },
        }
    }
}