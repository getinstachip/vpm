use std::env::Args;

#[derive(Debug, Default)]
pub struct Installer {
    author: String,
    package_name: String,
}

impl Installer {
    fn parse_package_details(package_details: String) -> (String, String) {
        let mut split = package_details.split("/");

        let author = split
            .next()
            .expect("Provided package author is empty")
            .to_string();

        let name = split
            .next()
            .expect("Provided package name is empty")
            .to_string();

        (author, name)
    }
}
