pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn factorial(i: u64) -> u64 {
    if i == 1 {
        i
    } else {
        i * factorial(i - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_factorial1() {
        let i = 1;
        assert_eq!(factorial(i), 1);
    }

    #[test]
    fn test_factorial10() {
        let i = 10;
        let result = factorial(i);
        println!("Result: {}", result);
        assert_eq!(result, 3628800);
    }
}

