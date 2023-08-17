use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use lofty::{read_from_path, Accessor, TaggedFileExt};

use crate::utils::fs::copy_file_with_parents;

#[derive(Debug)]
struct AudioBasicInfo {
    path: PathBuf,
    artist: String,
    album: String,
}

pub fn organizer_start(files_list: Vec<PathBuf>, target_folder_path: &Path) {
    let mut album2artist_map: HashMap<String, String> = HashMap::new();
    let mut audio_basic_info_list: Vec<AudioBasicInfo> = Vec::new();
    for audio in &files_list {
        let primary_artist = get_audio_primary_artist(audio);
        let album = get_audio_album(audio);
        if album != "Other" {
            if !album2artist_map.contains_key(&album) {
                album2artist_map.insert(album.clone(), primary_artist.clone());
            } else if album2artist_map.get(&album) != Some(&primary_artist) {
                album2artist_map.insert(album.clone(), "Various Artists".to_string());
            }
        }
        audio_basic_info_list.push(AudioBasicInfo {
            path: audio.to_path_buf(),
            artist: primary_artist,
            album: album,
        })
    }
    for audio_basic_info in &mut audio_basic_info_list {
        if audio_basic_info.album != "Other" {
            audio_basic_info.artist = album2artist_map
                .get(&audio_basic_info.album)
                .unwrap()
                .to_string();
        }
    }
    for audio_basic_info in audio_basic_info_list {
        let source_path = audio_basic_info.path.to_path_buf();
        let mut target_path = target_folder_path.to_path_buf();
        target_path.push(audio_basic_info.artist);
        target_path.push(audio_basic_info.album);
        target_path.push(audio_basic_info.path.file_name().unwrap());
        copy_file_with_parents(&source_path, &target_path);
        let mut lyric_source_path = audio_basic_info.path.parent().unwrap().to_path_buf();
        lyric_source_path.push(
            audio_basic_info
                .path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
                + ".lrc",
        );
        if lyric_source_path.exists() {
            target_path.pop();
            target_path.push(lyric_source_path.file_name().unwrap());
            copy_file_with_parents(&lyric_source_path, &target_path);
        }
    }
}

pub fn get_audio_album(audio_path: &Path) -> String {
    let tagged_file = read_from_path(audio_path).unwrap();
    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
    };
    let res = match tag.album().as_deref() {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => "Other".to_string(),
    };
    res
}

pub fn get_audio_primary_artist(audio_path: &Path) -> String {
    let tagged_file = read_from_path(audio_path).unwrap();
    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
    };
    let artist = match tag.artist().as_deref() {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => "Other".to_string(),
    };
    let primary_artist: Vec<&str> = artist.split('/').collect();
    primary_artist[0].to_string()
}
