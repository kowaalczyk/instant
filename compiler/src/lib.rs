pub mod stack;
pub mod jasmin;
pub mod llvm;

mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}