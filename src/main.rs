


pub mod tsdb;

fn main() {
    const TEST : &str = test();

    let message = hey(TEST.to_string());
    let message = message.to_uppercase();
    let message = message.replace("WORLD", "RUST");
    
    println!("{}", message);
}

const fn test () -> &'static str {
    "HELLO WORLD"
}

fn hey(p: String) -> String { 
    if 1 == 1 {
        format!("HELLO {p} WORLD")
    } else {
        format!("GOODBYE WORLD {p}")
    }
}