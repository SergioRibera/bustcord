mod animation;
mod parser;
mod style;
mod unit;

pub mod error;

pub use animation::*;
pub use declaration::Declaration;
pub use error::ParseError;
pub use style::StyleSheet;

pub(crate) use parser::*;
pub use unit::*;

use lexer::Token;

#[must_use]
pub fn css_to_rules(input: &str) -> Result<Vec<Rule<'_>>, Vec<SyntaxError>> {
    let tokens = Lexer::new(input).tokens();
    analyze_tokens(&tokens, input)?;
    let rules = Parser::new(tokens).parse();
    Ok(replace_vars(rules))
}

pub fn analyze(input: &str) -> Result<(), Vec<SyntaxError>> {
    let tokens = Lexer::new(input).tokens();
    let res = parser::analyze_tokens(&tokens, input);

    if !res.is_empty() {
        return Err(res);
    }

    Ok(())
}

pub fn analyze_tokens<'a>(
    tokens: &[Token<'a>],
    input: &'a str,
) -> Result<(), Vec<SyntaxError<'a>>> {
    let res = parser::analyze_tokens(&tokens, input);

    if !res.is_empty() {
        return Err(res);
    }

    Ok(())
}
