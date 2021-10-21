pub mod format;
pub mod http;
pub mod types;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
