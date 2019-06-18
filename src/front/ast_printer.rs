use crate::front::expr::*;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, terms: Vec<&Box<Expr>>) -> String {
        let mut strings: Vec<String> = Vec::new();
        strings.reserve(terms.len() * 2 + 2);

        strings.push(format!("({}", name.to_owned()));

        for boxed_expr in terms {
            strings.push(" ".to_owned());
            let expr = &**boxed_expr;
            strings.push(expr.accept(self));
        }
        strings.push(")".to_owned());
        strings.concat()
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_assign(&mut self, assign: &Assign) -> String {
        self.parenthesize(&format!("{} := ", assign.name.lexeme), vec![&assign.value])
    }

    fn visit_binary(&mut self, binary: &Binary) -> String {
        self.parenthesize(&binary.operator.lexeme, vec![&binary.left, &binary.right])
    }

    fn visit_call(&mut self, call: &Call) -> String {
        let mut terms: Vec<&Box<Expr>> = vec![&call.callee];
        let extend: Vec<Box<Expr>> = call
            .arguments
            .iter()
            .clone()
            .map(|a| Box::new(a.clone()))
            .collect();
        let rip: Vec<&Box<Expr>> = extend.iter().map(|lmao| lmao).collect();
        terms.extend(rip);
        self.parenthesize("call", terms)
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> String {
        self.parenthesize("group", vec![&grouping.expression])
    }

    fn visit_literal(&mut self, literal: &Literal) -> String {
        match literal {
            Literal::String(s) => s.to_owned(),
            Literal::Number(num) => num.to_string().to_owned(),
            Literal::Bool(b) => (if *b { "true" } else { "false" }).to_owned(),
            Literal::Nil => "Nil".to_owned(),
        }
    }

    fn visit_logical(&mut self, logical: &Binary) -> String {
        self.visit_binary(logical)
    }

    fn visit_unary(&mut self, unary: &Unary) -> String {
        self.parenthesize(&unary.operator.lexeme, vec![&unary.right])
    }

    fn visit_ternary(&mut self, ternary: &Ternary) -> String {
        self.parenthesize(
            "ternary",
            vec![
                &ternary.condition,
                &ternary.true_branch,
                &ternary.false_branch,
            ],
        )
    }

    fn visit_variable(&mut self, variable: &Variable) -> String {
        variable.name.lexeme.clone()
    }
}
