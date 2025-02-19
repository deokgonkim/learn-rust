//! This is Module TWO!
//!

/// This returns result of addition of two numbers
///
/// # Example
///
/// ```
/// use mod2;
///
/// let a = 789;
/// let b = 101;
///
/// assert_eq!(890, mod2::add(a, b));
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
