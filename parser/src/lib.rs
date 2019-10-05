// use lalrpop_util::lalrpop_mod;  // use with new lalrpop behaviour instead of pub mod calculator

pub mod ast;
pub mod calculator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
