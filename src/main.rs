use ganada::{is_korean, decompose};

fn main() {
    let text = "안녕 Rust!";
    println!("is_korean: {}", is_korean(text));
    println!("decompose: {:?}", decompose(text));
    
    println!("compose: {}", compose(decompose(text)));
}