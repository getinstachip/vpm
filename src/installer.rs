use std::env::Args;

#[derive(Debug, Default)]
pub struct Installer {
    package_name: String,
}

impl Installer {
    fn parse_package_details(package_details: String) -> String {
        let mut split = package_details.split("@");
        let name = split
            .next()
            .expect("Provided package name is empty")
            .to_string();
        name
    }
}
