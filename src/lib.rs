#![no_std]

pub mod common;
pub mod monitor;
pub mod platform;
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
