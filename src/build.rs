use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=ANTHROPIC_API_KEY");
    let key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
    println!("cargo:rustc-env=ANTHROPIC_API_KEY={}", key);
}
