use crate::parser::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    ValueInitialization{symbol: String, value: Expression},
    ValueAssignment{symbol: String, value: Expression}
}