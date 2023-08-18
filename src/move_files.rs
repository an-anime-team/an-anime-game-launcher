use std::path::Path;
use std::io::Result;

/// Move files from one folder to another
pub fn move_files(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    for entry in from.as_ref().read_dir()?.flatten() {
        let source = entry.path();
        let target = to.as_ref().join(entry.file_name());

        if std::fs::rename(&source, &target).is_err() {
            if source.is_dir() {
                std::fs::create_dir_all(&target)
                    .and_then(|_| move_files(&source, &target))
                    .and_then(|_| std::fs::remove_dir_all(&source))?;
            }

            else if source.is_symlink() {
                std::fs::read_link(&source)
                    .and_then(|link_target| std::os::unix::fs::symlink(link_target, &target))
                    .and_then(|_| std::fs::remove_file(&source))?;
            }

            else {
                std::fs::copy(&source, &target)
                    .and_then(|_| std::fs::remove_file(&source))?;
            } 
        }
    }

    Ok(())
}
