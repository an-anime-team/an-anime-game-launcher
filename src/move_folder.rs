use std::path::Path;

pub fn move_folder(from: &Path, to: &Path) -> std::io::Result<()> {
    if !to.exists() {
        std::fs::create_dir_all(to)?;
    }

    for entry in from.read_dir()?.flatten() {
        let to_path = to.join(entry.file_name());

        if entry.metadata()?.is_dir() {
            move_folder(&entry.path(), &to_path)?;
        }

        else if entry.metadata()?.is_file() {
            std::fs::copy(entry.path(), to_path)?;
            std::fs::remove_file(entry.path())?;
        }

        // TODO: symlinks?
    }

    std::fs::remove_dir_all(from)?;

    Ok(())
}
