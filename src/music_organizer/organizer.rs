use std::{
    collections::HashMap,
    fmt::Write,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use lofty::{read_from_path, Accessor, TaggedFileExt};

use crate::utils::fs::copy_file_with_parents;

#[derive(Debug)]
struct AudioBasicInfo {
    path: PathBuf,
    artist: String,
    album: String,
}

struct FileCopyUnit {
    from: PathBuf,
    to: PathBuf,
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
    let mut all_file_copy_task: Vec<FileCopyUnit> = Vec::new();
    for audio_basic_info in audio_basic_info_list {
        let source_path = audio_basic_info.path.to_path_buf();
        let mut target_path = target_folder_path.to_path_buf();
        target_path.push(audio_basic_info.artist);
        target_path.push(audio_basic_info.album);
        target_path.push(audio_basic_info.path.file_name().unwrap());
        all_file_copy_task.push(FileCopyUnit {
            from: source_path.to_path_buf(),
            to: target_path.to_path_buf(),
        });
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
            all_file_copy_task.push(FileCopyUnit {
                from: lyric_source_path.to_path_buf(),
                to: target_path.to_path_buf(),
            });
        }
    }
    file_copy_run(all_file_copy_task);
}

fn file_copy_run(file_copy_unit_list: Vec<FileCopyUnit>) {
    let total_size = file_copy_unit_list.len() as u64;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    for (index, file_copy_unit) in file_copy_unit_list.iter().enumerate() {
        let new = (index + 1) as u64;
        copy_file_with_parents(&file_copy_unit.from, &file_copy_unit.to);
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }
    pb.finish_with_message("file copy completed");
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
