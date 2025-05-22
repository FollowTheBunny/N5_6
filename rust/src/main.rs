use crate::ast::lexer::Lexer;
use crate::ast::parser::Parser;
use crate::ast::Ast; // Ensure this path is correct

mod ast;
// com
fn main() {
    let input = "2 + 3 * (2 + 4)";
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    let mut ast: Ast = Ast::new();
    let mut parser = Parser::new(tokens);
    loop {
        match parser.next_statement() {
            Some(stmt) => ast.add_statement(stmt),
            None => break,
        }
    }
    ast.visualize();
    // Evaluate the AST
    let result = ast.evaluate();
    if let Some(result) = result {
        println!("Result: {}", result);
    } else {
        println!("No result");
    }
}
