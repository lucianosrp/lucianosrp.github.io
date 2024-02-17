use rand::{thread_rng, Rng};

fn foo() -> Option<String> {
    let mut rng = thread_rng();

    if rng.gen_range(0..10) == 1 {
        Some(String::from("Hello"))
    } else {
        None
    }
}

fn bar(s: Option<String>) -> String {
    s.unwrap_or(String::from("")).to_uppercase()
}

fn main() {
    let random_val = foo();
    let res = vec!["HELLO".to_string(), "".to_string()];
    assert!(res.contains(&bar(random_val)))
}
