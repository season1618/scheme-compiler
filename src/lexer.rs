use Token::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Bool(bool),
    Number(i32),
    OpenPar,
    ClosePar,
    Period,
}

const OPERATORS: [char; 8] = ['=', '!', '<', '>', '+', '-', '*', '/'];

fn is_operator(c: char) -> bool {
    for operator in OPERATORS {
        if c == operator {
            return true;
        }
    }
    false
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut src_iter = src.chars();
    while let Some(mut c) = src_iter.next() {
        if c.is_whitespace() {
            continue;
        }

        if c.is_ascii_alphabetic() || is_operator(c) {
            let mut ident = c.to_string();
            while let Some(d) = src_iter.next() {
                if d.is_ascii_alphanumeric() || is_operator(d) {
                    ident.push(d);
                } else {
                    c = d;
                    break;
                }
            }
            token_list.push(Ident(ident));
        }

        if c.is_numeric() {
            let mut number = c.to_digit(10).unwrap();
            while let Some(d) = src_iter.next() {
                if d.is_numeric() {
                    number = 10 * number + d.to_digit(10).unwrap();
                } else {
                    c = d;
                    break;
                }
            }
            token_list.push(Number(number as i32));
        }
        
        if c == '(' {
            token_list.push(OpenPar);
            continue;
        }
        if c == ')' {
            token_list.push(ClosePar);
            continue;
        }
        if c == '.' {
            token_list.push(Period);
            continue;
        }
    }
    println!("{:?}", token_list);
    token_list
}