pub fn hello() {
    // function has to be public to access from outside
    println!("Other Greetings Hello, world!");
}

pub mod greetings {
    pub fn hello() {
        // function has to be public to access from outside
        println!("Other Greetings Hello, world!");
    }
}
