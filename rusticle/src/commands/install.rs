use crate::commands::Command;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

pub struct Install {
    pub package: String,
    pub temp: bool,
}

#[derive(Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
    functions: Vec<Function>,
}

#[derive(Deserialize, Serialize)]
struct Function {
    name: String,
    params: Vec<String>,
    body: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct PackagesLock {
    packages: Vec<Package>,
}

impl Command for Install {
    fn execute(&self) {
        println!("> Installing package: {}", self.package);

        let mut packages_lock = String::new();
        let packages_lock_path: &str;

        if !self.temp {
            packages_lock_path = "rusticle.lock";
        } else {
            packages_lock_path = "rusticle.temp.lock";
        }

        // Check if ructicle.lock exists, if not create it with an empty array
        if let Ok(mut file) = fs::File::open(packages_lock_path) {
            file.read_to_string(&mut packages_lock).expect("Unable to read ructicle.lock");
        } else {
            let mut file = fs::File::create(packages_lock_path).expect("Unable to create ructicle.lock");
            file.write_all(b"{\"packages\": []}").expect("Unable to write to ructicle.lock");
        }

        let mut packages_lock_data: PackagesLock = if !packages_lock.is_empty() {
            serde_json::from_str(&packages_lock).expect("Unable to parse ructicle.lock")
        } else {
            PackagesLock { packages: Vec::new() }
        };

        if packages_lock_data.packages.iter().any(|p: &Package| p.name == self.package) {
            println!("> Package '{}' is already installed.", self.package);
            return;
        }

        let client: Client = Client::new();
        let url: String = format!("http://127.0.0.1:8080/packages/{}", self.package);

        match client.get(&url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Package>() {
                        Ok(package) => {
                            let success: String = format!("> Package '{}' version '{}' installed successfully.", package.name, package.version);
                            packages_lock_data.packages.push(package);
                            let packages_lock_content: String = serde_json::to_string_pretty(&packages_lock_data).expect("Unable to serialize ructicle.lock");
                            let mut file: fs::File = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(packages_lock_path)
                                .expect("Unable to open ructicle.lock");
                            file.write_all(packages_lock_content.as_bytes()).expect("Unable to write to ructicle.lock");
                            println!("{}", success);
                        }
                        Err(err) => {
                            eprintln!("> Failed to parse package details or package doesn't exist: {}", err);
                        }
                    }
                } else {
                    eprintln!("> Failed to fetch package: {}", response.status());
                }
            }
            Err(err) => {
                eprintln!("> Failed to request package: {}", err);
            }
        }
    }
}