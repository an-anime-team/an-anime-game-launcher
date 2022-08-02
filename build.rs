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

fn blp_process_dir(dir: String) {
    let source_dir = format!("assets/ui/{}", &dir).replace("//", "/");
    let dist_dir = format!("assets/ui/.dist/{}", &dir).replace("//", "/");

    if let Ok(entries) = read_dir(&source_dir) {
        if let Err(_) = read_dir(&dist_dir) {
            create_dir_all(&dist_dir).expect("UI dist dir couldn't be created");
        }

        // println!("cargo:rerun-if-changed={}/*.blp", &source_dir);

        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    let entry_path = entry.path().to_str().unwrap().to_string();
                    let entry_filename = entry.file_name().to_str().unwrap().to_string();

                    if metadata.is_file() {
                        let entry_dist_path = format!("{}/{}.ui", &dist_dir, &entry_filename[..entry_filename.len() - 4]);

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

                    else if metadata.is_dir() && &entry_filename[0..1] != "." {
                        blp_process_dir(format!("{}/{}", &dir, &entry_filename));
                    }
                }
            }
        }
    }
}

fn main() {
    blp_process_dir(String::new());

    if let Ok(_) = read_to_string("assets/resources.xml") {
        gtk4::gio::compile_resources(
            "assets",
            "assets/resources.xml",
            ".assets.gresource",
        );
    }
}
