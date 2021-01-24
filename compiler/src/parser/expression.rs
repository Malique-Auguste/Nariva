use crate::parser::operator::Operator;

pub enum Expression {
    Binary { lhs: Box<Expression>, symbol: Operator, rhs: Box<Expression> },
    Number { inner: u64 },
    Unary { symbol: Operator, inner: Box<Expression> }
}

impl Expression {
    pub fn resolve(&self) -> u64 {
        match self {
            Expression::Binary{lhs, symbol, rhs} => {
                match symbol {
                    Operator::Plus => lhs.resolve() + rhs.resolve(),
                    Operator::Minus => lhs.resolve() - rhs.resolve(),
                    _ => unimplemented!()
                }
            },

            Expression::Number{inner} => *inner,
            
            Expression::Unary{symbol, inner} => {
                match symbol {
                    Operator::Bang => !inner.resolve(),
                    _=> unimplemented!()
                }
            }
        }
    }
}