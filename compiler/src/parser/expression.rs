use crate::parser::operator::Operator;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary { lhs: Box<Expression>, operator: Operator, rhs: Box<Expression> },
    U64 { inner: u64 },
    F64 { inner: f64 },
    Str { inner: String },
    Unary { operator: Operator, inner: Box<Expression> }
}

/*
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
*/