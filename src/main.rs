pub mod lexer;
pub mod parser;
pub mod codegen;

use std::env;
use std::fs;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::codegen::gen_asm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src_name: &str = &args[1];
    let dst_name;
    if src_name.ends_with(".scm") {
        let len = src_name.len();
        dst_name = format!("{}.s", &src_name[..len-4]);
    } else {
        println!("{} is not scheme file", src_name);
        return;
    }

    let src_code = &fs::read_to_string(src_name).expect("could not read the source file");
    let token_list = tokenize(src_code);
    let node_list = parse(token_list);
    gen_asm(node_list, dst_name);
}