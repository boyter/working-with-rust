// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
// Answer 233168
pub fn euler_1() -> i32 {
    let mut sum = 0;

    for i in 1..1000 {
        if i % 3 == 0 {
            sum += i;
        }
        else if i % 5 == 0 {
            sum += i;
        }
    }

    sum
}


#[cfg(test)]
mod main {
    use super::*;

    #[test]
    fn test_euler_1() {
         assert_eq!(233168, euler_1());
    }
}

