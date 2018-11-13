
pub fn hello() {
    println!("Crate Learning Hello World");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
