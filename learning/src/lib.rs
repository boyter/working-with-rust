pub fn cratehello() -> String {
    ("Crate Hello, world!").to_string()
}

#[cfg(test)] // only compiles when runing tests
mod tests {
    use super::cratehello; // import root hello function

    #[test]
    fn test_hello() {
        assert_eq!(cratehello(), "Crate Hello, world!");
    }
}
