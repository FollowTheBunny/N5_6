use crate::ast::lexer::{Token, TokenKind};
use crate::ast::{ASTBinaryOperator, ASTBinaryOperatorKind, ASTExpression, ASTStatement};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens
                .iter()
                .filter(|token| token.kind != TokenKind::Whitespace)
                .map(|token| token.clone())
                .collect(),
            current: 0,
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        return self.parse_statement();
    }

    fn parse_statement(&mut self) -> Option<ASTStatement> {
        let token = self.current()?;
        let expr = self.parse_expression()?;
        return Some(ASTStatement::expression(expr));
    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {
        return self.parse_binary_expression(0);
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<ASTExpression> {
        let mut left = self.parse_primary_expression()?;

        while let Some(operator) = self.parse_binary_operator() {
            self.consume();
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            let right = self.parse_binary_expression(operator_precedence)?;
            left = ASTExpression::binary(operator, left, right);
        }

        return Some(left);
    }

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        let token = self.current()?;
        let kind = match token.kind {
            TokenKind::Plus => Some(ASTBinaryOperatorKind::Plus),
            TokenKind::Minus => Some(ASTBinaryOperatorKind::Minus),
            TokenKind::Asterisk => Some(ASTBinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(ASTBinaryOperatorKind::Divide),
            TokenKind::Pow => Some(ASTBinaryOperatorKind::Pow),
            TokenKind::IntegerDivide => Some(ASTBinaryOperatorKind::IntegerDivide),
            _ => None,
        };
        return kind.map(|kind| ASTBinaryOperator::new(kind, token.clone()));
    }
    fn parse_primary_expression(&mut self) -> Option<ASTExpression> {
        let token = self.consume()?;
        return match &token.kind {
            TokenKind::Real(number) => Some(ASTExpression::number(*number)),
            TokenKind::Variable(name) => Some(ASTExpression::variable(name.clone())),
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                let token = self.consume()?;
                if token.kind != TokenKind::RightParen {
                    panic!("Expected right paren");
                }
                Some(ASTExpression::parenthesized(expr))
            }
            TokenKind::Begin => {
                let expr = self.parse_expression()?;
                let token = self.consume()?;
                if token.kind != TokenKind::End {
                    panic!("Expected right paren");
                }
                Some(ASTExpression::start_end(expr))
            }
            _ => None,
        };
    }

    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1)?;
        return Some(token);
    }
}
