use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}