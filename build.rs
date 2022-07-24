use std::process::{Command, Stdio};
use std::fs::{self, read_dir, create_dir_all, read_to_string};
use std::path::Path;

fn compile_blueprint<T: ToString>(path: T) -> Result<String, String> {
    // python blueprint-compiler/blueprint-compiler.py compile ui/main.blp
    let output = Command::new("python3")
        .arg("blueprint-compiler/blueprint-compiler.py")
        .arg("compile")
        .arg(path.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            }

            else {
                Err(String::from_utf8(output.stdout).unwrap())
            }
        },
        Err(err) => Err(err.to_string())
    }
}

fn main() {
    if let Ok(_) = read_to_string("assets/resources.xml") {
        gtk4::gio::compile_resources(
            "assets",
            "assets/resources.xml",
            ".assets.gresource",
        );
    }

    if let Ok(entries) = read_dir("assets/ui") {
        if let Err(_) = read_dir("assets/ui/.dist") {
            create_dir_all("assets/ui/.dist").expect("UI dist dir couldn't be created");
        }

        println!("cargo:rerun-if-changed=assets/ui/*.blp");

        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let entry_path = entry.path().to_str().unwrap().to_string();
                        let entry_filename = entry.file_name().to_str().unwrap().to_string();

                        let entry_dist_path = format!("assets/ui/.dist/{}.ui", &entry_filename[..entry_filename.len() - 4]);

                        match compile_blueprint(&entry_path) {
                            Ok(xml) => {
                                let result = fs::write(entry_dist_path, xml);

                                if let Err(err) = result {
                                    println!("cargo:warning=Couldn't write compiled XML UI: {}", err);
                                }
                            },
                            Err(err) => {
                                if Path::new(&entry_dist_path).exists() {
                                    fs::remove_file(entry_dist_path).expect("Couldn't remove broken file");
                                }

                                println!("cargo:warning=Couldn't compile {}: {}", entry_path, err);
                            }
                        }
                    }
                }
            }
        }
    }
}
