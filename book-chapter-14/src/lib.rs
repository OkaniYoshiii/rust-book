//! # App
//! 
//! A simple application to demonstrate the power of
//! cargo doc command

/// Adds two numbers together
/// 
/// # Examples
/// 
/// ```
/// let age: i64 = 32;
/// let other_age: i64 = 64;
/// let sum = app::add(age, other_age);
/// 
/// assert_eq!(sum, 96);
/// ```
pub fn add(first: i64, second: i64) -> i64 {
    first + second
}

pub use crate::test::tester;
pub use crate::other::other;

pub mod test {
    pub fn tester() {
        unimplemented!();
    }
}

pub mod other {
    use super::test;

    pub fn other() {
        test::tester();
    }
}