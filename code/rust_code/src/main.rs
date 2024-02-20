use rand::{thread_rng, Rng};

fn random_error() -> Result<String, ()> {
    let mut rng = thread_rng();

    if rng.gen_range(0..10) == 1 {
        Ok(String::from("Result A"))
    } else {
        Err(())
    }
}

fn main() {
    let val = random_error();
    match val {
        Ok(result) => println!("We safely got the result, {result}"),
        Err(_) => println!("We received an error!"),
    }
}
