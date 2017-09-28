use std::collections::HashMap;


fn main() {

    let str1 = String::from("Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.");
    let str2 = String::from("Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. Featuring zero-cost abstractions move semantics guaranteed memory safety threads without data races trait-based generics pattern matching type inference minimal runtime efficient C bindings");
    let con1 = concordance(&str1);
    let con2 = concordance(&str2);

    let rel = relation(&con1, &con2);

    println!("{}", rel);
}

fn magnitude(concordance: &HashMap<&str, u32>) -> f64 {
    let mut total = 0.0;

    for (_, value) in concordance {
        total += (*value as f64).powf(2.0);
    }
    
    (total as f64).sqrt()
}

fn relation(concordance1: &HashMap<&str, u32>, concordance2: &HashMap<&str, u32>) -> f64 {
    let mut topvalue = 0;

    for (key, value) in concordance1 {
        let found = concordance2.get(key);

        if found.is_some() {
            topvalue += value * found.unwrap();
        }
    }

    return (topvalue as f64) / (magnitude(concordance1) * magnitude(concordance2))
}

fn concordance(document: &str) -> HashMap<&str, u32> {
    let mut concordance = HashMap::new();
    for word in document.split_whitespace() {
        let count = concordance.entry(word).or_insert(0);
        *count += 1;
    }

    concordance
}