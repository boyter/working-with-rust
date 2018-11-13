mod othergreetings;
mod deepgreetings;

fn main() {
    greet();
    greetings::hello();
    phrases::greetings::hello();
    phrases::greet();
    phrases::greetings::prihello();
    othergreetings::hello();
    othergreetings::greetings::hello();
    deepgreetings::hello();
    deepgreetings::greetings::hello();
    deepgreetings::hidden::hiddenhello();
}

fn greet() -> String {
    "Hello, world!".to_string()
}

mod greetings {
    // ⭐️ By default, everything inside a module is private
    pub fn hello() { // ⭐️ So function has to be public to access from outside
        println!("Hello, world!");
    }
}

mod phrases {
    pub mod greetings {
        pub fn hello() {
            println!("Hello, world!");
        }

        pub fn prihello() {
            super::hello();
        }
    }

    pub fn greet() {
        self::hello();
    }

    fn hello() {
        println!("Private Hello world");
    }

}

#[cfg(test)] // only compiles when running tests
mod tests {
    use super::greet; // import root greet function

    #[test]
    fn test_greet() {
        assert_eq!("Hello, world!", greet());
    }
}