// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?
// Answer 104743
pub fn euler_7() -> i32 {
    let mut count = 0;
    let mut i = 2;

    loop {
        let is_prime = is_prime(i);

        if is_prime {
            count += 1;
        }

        if count == 10001 {
            break;
        }

        i += 1;
    }

    i
}

fn is_prime(x: i32) -> bool {
    if x == 2 {
        return true;
    }
    if x == 3 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    if x % 3 == 0 {
        return false;
    }

    let mut i = 5;
    let mut w = 2;

    while i * i <= x {
        if x % i == 0 {
            return false;
        }

        i += w;
        w = 6 - w;
    }

    true
}


#[cfg(test)]
mod main {
    use super::*;

    #[test]
    fn test_euler_7() {
         assert_eq!(104743, euler_7());
    }
}


