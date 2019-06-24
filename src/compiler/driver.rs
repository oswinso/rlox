use crate::compiler::{Scanner, TokenKind};
use crate::utils::PrettyPrinter;

pub fn compile(src: &str) {
    let mut pretty_printer = PrettyPrinter::new(String::new());

    let mut line = 0;
    let mut scanner = Scanner::new(src);
    loop {
        let mut token = scanner.scan_token();

        if token.position.line != line {
            line = token.position.line;
            pretty_printer.line_number(Some(line)).print();
        } else {
            pretty_printer.line_number(None).print();
        }
        pretty_printer.token(&token).newline().print();

        if token.ty == TokenKind::EOF {
            break;
        }
    }
}
