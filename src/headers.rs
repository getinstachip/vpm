pub (crate) fn generate_header(content: &String, file_name: &str) -> String {
    let mut header = String::new();

    // Add header guard
    let guard_name = file_name.replace(".", "_").to_uppercase();
    header.push_str(&format!("`ifndef {}\n", guard_name));
    header.push_str(&format!("`define {}\n\n", guard_name));

    // Parse the content for constants, parameters, and potential macros
    for line in content.lines() {
        if line.trim().starts_with("parameter") || line.trim().starts_with("localparam") {
            header.push_str(&format!("`define {}\n", line.trim()));
        } else if line.contains("`define") {
            header.push_str(&format!("{}\n", line.trim()));
        }
    }

    // Close the header guard
    header.push_str(&format!("\n`endif // {}\n", guard_name));
    header
}