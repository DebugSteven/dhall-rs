extern crate bytecount;
extern crate lalrpop_util;
#[macro_use]
extern crate nom;
extern crate term_painter;

mod core;
pub use core::*;
pub mod grammar;
mod grammar_util;
pub mod lexer;
pub mod parser;

use std::io::{self, Read};

fn print_error(message: &str, source: &str, start: usize, end: usize) {
    let line_number = bytecount::count(source[..start].as_bytes(), '\n' as u8);
    let line_start = source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line_end = source[end..].find('\n').unwrap_or(0) + end;
    let context_prefix = &source[line_start..start];
    let context_highlighted = &source[start..end];
    let context_suffix = &source[end..line_end];

    let line_number_str = line_number.to_string();
    let line_number_width = line_number_str.len();

    use term_painter::ToStyle;
    let err_style = term_painter::Color::Red;
    let bold = term_painter::Attr::Bold;

    bold.with(|| {
        err_style.with(|| {
            print!("error: ");
        });
        println!("{}", message);
    });
    bold.with(|| {
        print!("  -->");
    });
    println!(" {}:{}:0", "(stdin)", line_number);
    bold.with(|| {
        println!("{:w$} |", "", w = line_number_width);
        print!("{} |", line_number_str);
    });
    print!(" {}", context_prefix);
    bold.with(|| {
        err_style.with(|| {
            print!("{}", context_highlighted);
        });
    });
    println!("{}", context_suffix);
    bold.with(|| {
        print!("{:w$} |", "", w = line_number_width);
        err_style.with(|| {
            println!(" {:so$}{:^>ew$}", "", "",
                     so = source[line_start..start].chars().count(),
                     ew = ::std::cmp::max(1, source[start..end].chars().count()));
        });
    });
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    match parser::parse_expr(&buffer) {
        Ok(e) => println!("{:?}", e),
        Err(lalrpop_util::ParseError::User { error: lexer::LexicalError::Error(pos, e) }) => {
            print_error(&format!("Unexpected token {:?}", e), &buffer, pos, pos);
        }
        Err(lalrpop_util::ParseError::UnrecognizedToken { token: Some((start, t, end)), expected: e }) => {
            print_error(&format!("Unrecognized token {:?}", t), &buffer, start, end);
            if e.len() > 0 {
                println!("Expected {:?}", e);
            }
        }
        Err(e) => {
            print_error(&format!("Parser error {:?}", e), &buffer, 0, 0);
        }
    }

    /*
    expr <- case exprFromText (Directed "(stdin)" 0 0 0 0) inText of
        Left  err  -> Control.Exception.throwIO err
        Right expr -> return expr

    expr' <- load expr

    typeExpr <- case Dhall.TypeCheck.typeOf expr' of
        Left  err      -> Control.Exception.throwIO err
        Right typeExpr -> return typeExpr
    Data.Text.Lazy.IO.hPutStrLn stderr (pretty (normalize typeExpr))
    Data.Text.Lazy.IO.hPutStrLn stderr mempty
    Data.Text.Lazy.IO.putStrLn (pretty (normalize expr')) )
    */
}
