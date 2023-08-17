use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn list_files_in_folder(folder_path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut file_paths = Vec::new();
    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            file_paths.push(path.clone());
        } else if path.is_dir() {
            let subfolder_files = list_files_in_folder(&path)?;
            file_paths.extend(subfolder_files);
        }
    }

    Ok(file_paths)
}

pub fn copy_file_with_parents(
    source_path: &Path,
    target_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建目标目录的父文件夹（如果不存在）
    if let Some(parent_dir) = target_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    // 复制文件（覆盖同名文件）
    fs::copy(source_path, target_path)?;

    Ok(())
}

pub fn is_audio_file(file_path: &Path) -> bool {
    let kind = infer::get_from_path(file_path)
        .expect("file read successfully")
        .expect("file type is known");
    let file_type: &str = kind.mime_type();
    file_type.split('/').collect::<Vec<&str>>()[0] == "audio"
}
