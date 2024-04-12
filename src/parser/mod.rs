use crate::ast::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub zexpr, "/parser/zexpr.rs");

#[cfg(test)]
mod test {
    use crate::parser::zexpr;

    #[test]
    pub fn testa() {
        assert!(zexpr::TermParser::new().parse("(20)").is_ok());
        println!("***********a simple test")
    }
}
