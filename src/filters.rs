use scanner::Token;
use super::Context;

pub struct FilterExpression {
    var: String,
    //filters: 
}

impl FilterExpression {
    pub fn new(token: &Token) -> FilterExpression {
        FilterExpression {var: token.content.trim().to_string()}
    }

    pub fn var(&self) -> &str { &self.var }

    pub fn render(&self, context: &Context) -> String {
        format!("{}", context.get(&self.var).unwrap())
    }
}