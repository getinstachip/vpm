use bytes::Bytes;
use std::fs::{self};
use std::path::Path;
use std::sync::mpsc::Sender;

use crate::util::TaskAllocator;

pub type PackageBytes = (String, Bytes); // Package destination, package bytes

#[derive(Clone)]
pub struct InstallContext {
    pub client: reqwest::Client,
    pub bytes_sender: Sender<PackageBytes>,
}

#[derive(Debug)]
pub struct Installer;
impl Installer {
    pub fn install_package(context: InstallContext) {
        TaskAllocater::add_task(async move {
            let package_bytes = HttpRequest::get_bytes(context.client.clone())
                .await
                .unwrap();

            let package_destination = format!("{}", *CACHE_DIRECTORY);

            context
                .bytes_sender
                .send((package_destination, package_bytes))
                .unwrap();
        });
    }

    pub fn create_modules_dir() {
        if Path::new("./vpm_modules").exists() {
            return;
        }

        fs::create_dir("./vpm_modules").expect("Failed to create vpm modules folder");
    }
}
