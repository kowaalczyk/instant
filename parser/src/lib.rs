// use lalrpop_util::lalrpop_mod;  // use with new lalrpop behaviour instead of pub mod instant

pub mod ast;
pub mod instant;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
