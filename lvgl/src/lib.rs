#![no_std]

mod objx;
pub mod display;

pub use objx::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
