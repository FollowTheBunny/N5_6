use crate::ast::lexer::Token;
pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) -> () {
        let mut printer = ASTPrinter { indent: 1 };
        self.visit(&mut printer);
    }

    pub fn visualizeXML(&self) -> () {
        print!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        print!("<program>\n");
        let mut printer = ASTXMLPrinter { indent: 1 };
        self.visit(&mut printer);

        print!("</program>");
    }

    pub fn evaluate(&self) -> Option<f64> {
        let mut evaluator = ASTEvaluator::new();
        for statement in &self.statements {
            evaluator.visit_statement(statement);
        }
        evaluator.last_value
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
        }
    }
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }
    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number(number);
            }
            ASTExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            ASTExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
            ASTExpressionKind::StartEnd(expr) => {
                self.visit_start_end_expression(expr);
            }
            ASTExpressionKind::Variable(variable_name) => {
                let variable_expression = ASTVariableExpression::new(variable_name.clone());
                self.visit_variable(&variable_expression);
            }
        }
    }
    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_number(&mut self, number: &ASTNumberExpression);

    fn visit_variable(&mut self, variable: &ASTVariableExpression);

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression,
    ) {
        self.visit_expression(&parenthesized_expression.expression);
    }

    fn visit_start_end_expression(&mut self, start_end_expression: &ASTStartEndExpression) {
        self.visit_expression(&start_end_expression.expression);
    }
}

pub struct ASTPrinter {
    indent: usize,
}
const LEVEL_INDENT: usize = 2;

impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_with_indent("");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_statement(self, statement);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.print_with_indent("");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_expression(self, expression);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_number(&mut self, number: &ASTNumberExpression) {
        let number_str = if number.number.fract() == 0.0 {
            format!("{}", number.number) + ".0"
        } else {
            format!("{}", number.number)
        };
        self.print_with_indent(&format!("Real({})", number_str));
    }

    fn visit_variable(&mut self, variable: &ASTVariableExpression) {
        self.print_with_indent(&format!("Variable({})", variable.name));
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.print_with_indent(&format!("{:?}", binary_expression.operator.kind));

        self.print_with_indent("("); // Space added
        self.visit_expression(&binary_expression.left);

        self.print_with_indent(","); // Space added
        self.visit_expression(&binary_expression.right);
        self.print_with_indent(")"); // Space added
    }
    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression,
    ) {
        self.print_with_indent("(");
        self.indent += LEVEL_INDENT;
        self.visit_expression(&parenthesized_expression.expression);

        self.print_with_indent(")");
        self.indent -= LEVEL_INDENT;
    }

    fn visit_start_end_expression(&mut self, start_end_expression: &ASTStartEndExpression) {
        self.print_with_indent("{");
        // self.indent += LEVEL_INDENT;
        self.visit_expression(&start_end_expression.expression);

        self.print_with_indent("}");
        //self.indent -= LEVEL_INDENT;
    }
}

impl ASTPrinter {
    fn print_with_indent(&mut self, text: &str) {
        print!("{}{}", "".repeat(self.indent), text);
    }
}

pub struct ASTXMLPrinter {
    indent: usize,
}

impl ASTVisitor for ASTXMLPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_with_indent("");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_statement(self, statement);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.print_with_indent("");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_expression(self, expression);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_number(&mut self, number: &ASTNumberExpression) {
        let number_str = if number.number.fract() == 0.0 {
            format!("{}", number.number) + ".0"
        } else {
            format!("{}", number.number)
        };
        self.print_with_indent(&format!("<real>{}</real>\n", number_str));
    }

    fn visit_variable(&mut self, variable: &ASTVariableExpression) {
        self.print_with_indent(&format!("<variable>{}</variable>\n", variable.name));
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.print_with_indent(&format!("<{:?}>", binary_expression.operator.kind));

        self.print_with_indent("\n"); // Space added
        self.visit_expression(&binary_expression.left);

        self.print_with_indent("\n"); // Space added
        self.visit_expression(&binary_expression.right);
        self.print_with_indent(&format!("</{:?}>\n", binary_expression.operator.kind));
    }
    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression,
    ) {
        self.print_with_indent("<paren>");
        self.indent += LEVEL_INDENT;
        self.visit_expression(&parenthesized_expression.expression);

        self.print_with_indent("</paren>\n");
        self.indent -= LEVEL_INDENT;
    }

    fn visit_start_end_expression(&mut self, start_end_expression: &ASTStartEndExpression) {
        self.print_with_indent("{");
        // self.indent += LEVEL_INDENT;
        self.visit_expression(&start_end_expression.expression);

        self.print_with_indent("}");
        //self.indent -= LEVEL_INDENT;
    }
}

impl ASTXMLPrinter {
    fn print_with_indent(&mut self, text: &str) {
        print!("{}{}", "".repeat(self.indent), text);
    }
}

#[derive(Debug)]
pub enum ASTStatementKind {
    Expression(ASTExpression),
}

#[derive(Debug)]
pub struct ASTStatement {
    pub kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }
}

#[derive(Debug)]
pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
    StartEnd(ASTStartEndExpression),

    Variable(String),
}

#[derive(Debug)]
pub enum ASTBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Pow,
    IntegerDivide,
    Eof,
}

#[derive(Debug)]
pub struct ASTBinaryOperator {
    kind: ASTBinaryOperatorKind,
    token: Token,
}

impl ASTBinaryOperator {
    pub fn new(kind: ASTBinaryOperatorKind, token: Token) -> Self {
        ASTBinaryOperator { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Eof => 1,
            ASTBinaryOperatorKind::Plus => 1,
            ASTBinaryOperatorKind::Minus => 1,
            ASTBinaryOperatorKind::Multiply => 2,
            ASTBinaryOperatorKind::Divide => 2,
            ASTBinaryOperatorKind::IntegerDivide => 2,
            ASTBinaryOperatorKind::Pow => 3,
        }
    }
}

#[derive(Debug)]
pub struct ASTBinaryExpression {
    left: Box<ASTExpression>,
    operator: ASTBinaryOperator,
    right: Box<ASTExpression>,
}

#[derive(Debug)]
pub struct ASTNumberExpression {
    number: f64,
}

#[derive(Debug)]
pub struct ASTVariableExpression {
    name: String,
}
impl ASTVariableExpression {
    pub fn new(name: String) -> Self {
        ASTVariableExpression { name }
    }
}
#[derive(Debug)]
pub struct ASTParenthesizedExpression {
    expression: Box<ASTExpression>,
}
#[derive(Debug)]
pub struct ASTStartEndExpression {
    expression: Box<ASTExpression>,
}

#[derive(Debug)]
pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: f64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number }))
    }

    pub fn variable(name: String) -> Self {
        ASTExpression::new(ASTExpressionKind::Variable(name))
    }

    pub fn binary(operator: ASTBinaryOperator, left: ASTExpression, right: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Binary(ASTBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    pub fn parenthesized(expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Parenthesized(
            ASTParenthesizedExpression {
                expression: Box::new(expression),
            },
        ))
    }
    pub fn start_end(expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::StartEnd(ASTStartEndExpression {
            expression: Box::new(expression),
        }))
    }
}

use std::collections::HashMap;

pub struct ASTEvaluator {
    pub last_value: Option<f64>,
    pub variables: HashMap<String, f64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self {
            last_value: None,
            variables: HashMap::new(),
        }
    }
}

impl ASTVisitor for ASTEvaluator {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        ASTVisitor::do_visit_statement(self, statement);
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number(number);
            }
            ASTExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            ASTExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
            ASTExpressionKind::StartEnd(expr) => {
                self.visit_start_end_expression(expr);
            }
            ASTExpressionKind::Variable(variable_name) => {
                let variable_expression = ASTVariableExpression::new(variable_name.clone());
                if (variable_name == "x") {
                    self.last_value = Some(1.0 as f64);
                } else if (variable_name == "y") {
                    self.last_value = Some(3.0 as f64);
                } else {
                    self.visit_variable(&variable_expression);
                }
            }
        }
    }

    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.number as f64);
    }
    fn visit_variable(&mut self, variable: &ASTVariableExpression) {
        self.last_value = self.variables.get(&variable.name).cloned();
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        let left = self.last_value.unwrap();
        print!("{}", left);
        self.visit_expression(&binary_expression.right);
        let right = self.last_value.unwrap();

        self.last_value = Some(match binary_expression.operator.kind {
            ASTBinaryOperatorKind::Plus => left + right,
            ASTBinaryOperatorKind::Minus => left - right,
            ASTBinaryOperatorKind::Multiply => left * right,
            ASTBinaryOperatorKind::Divide => left / right,
            ASTBinaryOperatorKind::Pow => left.powf((right as u32).into()),
            ASTBinaryOperatorKind::IntegerDivide => left / right,
            ASTBinaryOperatorKind::Eof => 0.0,
            _ => panic!("Unhandled binary operator kind"),
        });
    }
}
