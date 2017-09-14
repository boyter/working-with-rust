// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
// Answer 6857
pub fn euler_3() -> i64 {
    let upper: i64 = (600851475143_f64.sqrt()) as i64;

    for i in (2..upper).rev() {

        if 600851475143 % i == 0 {
            let is_prime = is_prime(i);

            if is_prime {
                return i;
            }
        }
    }

    0
}

fn is_prime(x: i64) -> bool {
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
    fn test_euler_3() {
         assert_eq!(6857, euler_3());
    }
}


