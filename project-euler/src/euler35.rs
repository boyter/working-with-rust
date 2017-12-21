// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
// How many circular primes are there below one million?
pub fn euler_35() -> i32 {
    let mut count = 0;

    for i in 1..1000000 {
        if !is_excluded(i) && is_prime(i) {
            let mut rot = rotate_number(i);
            let mut is_circular = true;

            for _ in 1..i.to_string().len() {
                if !is_prime(rot) {
                    is_circular = false;
                    break;
                }
                rot = rotate_number(rot);
            }

            if is_circular {
                // println!("{}", i);
                count += 1;
            }
        }
    }

    count
}

fn is_excluded(x: i32) -> bool {
    if x == 2 {
        return false;
    }

    if contains_digit(x, 2) || contains_digit(x, 4) || contains_digit(x, 5) || contains_digit(x, 6) || contains_digit(x, 8) || contains_digit(x, 0) {
        return true;
    }

    false
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

fn contains_digit(haystack: i32, needle: i32) -> bool {
    let mut this_number = haystack;
    let mut this_digit;

    while this_number != 0 {
        this_digit = this_number % 10;
        this_number = this_number / 10;

        if this_digit == needle {
            return true;
        }
    }

    return false;
}

fn rotate_number(x: i32) -> i32 {
    if x >= 1000000000 { return (x - ( (x / 1000000000) * 1000000000)) * 10 + (x / 1000000000); }
    if x >= 100000000 { return (x - ( (x / 100000000) * 100000000)) * 10 + (x / 100000000); }
    if x >= 10000000 { return (x - ( (x / 10000000) * 10000000)) * 10 + (x / 10000000); }
    if x >= 1000000 { return (x - ( (x / 1000000) * 1000000)) * 10 + (x / 1000000); }
    if x >= 100000 { return (x - ( (x / 100000) * 100000)) * 10 + (x / 100000); }
    if x >= 10000 { return (x - ( (x / 10000) * 10000)) * 10 + (x / 10000); }
    if x >= 1000 { return (x - ( (x / 1000) * 1000)) * 10 + (x / 1000); }
    if x >= 100 { return (x - ( (x / 100) * 100)) * 10 + (x / 100); }
    if x >= 10 { return (x - ( (x / 10) * 10)) * 10 + (x / 10); }

    x
}


#[cfg(test)]
mod main {
    use super::*;

    #[test]
    fn test_euler_35() {
         assert_eq!(55, euler_35());
    }

    #[test]
    fn test_is_excluded() {
        assert_eq!(true, is_excluded(22));
        assert_eq!(false, is_excluded(2));
    } 

    #[test]
    fn test_rotate_number() {
        assert_eq!(1, rotate_number(1));
        assert_eq!(2, rotate_number(2));
        assert_eq!(199, rotate_number(919));
        assert_eq!(919, rotate_number(991));
        assert_eq!(999, rotate_number(999));
        assert_eq!(112, rotate_number(211));
        assert_eq!(9991, rotate_number(1999));
        assert_eq!(9919, rotate_number(9991));
    }
}
