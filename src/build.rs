use std::env;

#[allow(dead_code)]
fn main() {
    let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| "DEFAULT_KEY".to_string());
    println!("cargo:rustc-env=ANTHROPIC_API_KEY={}", api_key);
}
