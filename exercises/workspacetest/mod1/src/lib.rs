//! This is Module One
//!

/// This returns addition
///
/// # Example
///
/// ```
/// use mod1::add;
///
/// let a = 123;
/// let b = 456;
///
/// assert_eq!(579, add(a, b))
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

