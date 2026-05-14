use {
    rfd::AsyncFileDialog,
    std::{env, path::PathBuf},
};

pub async fn get_directory() -> Option<PathBuf> {
    let mut dialog = AsyncFileDialog::new().set_title("Choose directory which will be scanned");

    if let Ok(path) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        dialog = dialog.set_directory(PathBuf::from(path));
    }

    dialog
        .pick_folder()
        .await
        .map(|handle| handle.path().to_path_buf())
}
