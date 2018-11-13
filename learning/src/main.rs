fn main() {
    greet();
    greetings::hello();
    phrases::greetings::hello();
    phrases::greet();
}

fn greet() -> String {
    "Hello, world!".to_string()
}

#[test] // test attribute indicates, this is a test function
fn test_greet() {
    assert_eq!("Hello, world!", greet())
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
    }

    pub fn greet() {
        self::hello();
    }

    fn hello() {
        println!("Hello world");
    }

}