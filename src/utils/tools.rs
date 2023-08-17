use std::path::{Path, PathBuf};

use super::fs::list_files_in_folder;

pub fn contains_tv_keywords(album: &str) -> bool {
    let keywords = vec!["影视原声", "电视剧原声"];
    for keyword in keywords {
        if album.contains(keyword) {
            return true;
        }
    }
    false
}

pub fn get_audio_file_list(folder_path: &Path) -> Vec<PathBuf> {
    let file_list = list_files_in_folder(folder_path).unwrap();
    let mut audio_file_list: Vec<PathBuf> = Vec::new();
    for file in file_list {
        if let Some(kind) = infer::get_from_path(file.as_path()).expect("file read successfully") {
            let mime = kind.mime_type();
            if (mime.contains("audio")) {
                audio_file_list.push(file);
            }
        }
    }
    audio_file_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_tv_keywords() {
        assert_eq!(contains_tv_keywords("test 影视原声带"), true);
        assert_eq!(contains_tv_keywords("test 电视剧原声带"), true);
        assert_eq!(contains_tv_keywords("test"), false);
    }
}
