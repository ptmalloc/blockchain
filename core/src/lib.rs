pub mod block;
pub mod blockchain;
pub mod filewrite;
pub mod netrpc;
pub mod exchangeinfo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
