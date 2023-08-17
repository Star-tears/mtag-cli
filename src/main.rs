use std::path::PathBuf;

use clap::Parser;

use mtag_cli::{music_organizer::organizer::organizer_start, utils::tools::get_audio_file_list};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "MUSIC_FOLDER", default_value = ".")]
    music_folder_path: PathBuf,
    #[arg(value_name = "TARGET_FOLDER", default_value = "Music")]
    target_folder_path: PathBuf,
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let audio_file_list = get_audio_file_list(&cli.music_folder_path);
    organizer_start(audio_file_list, &cli.target_folder_path);
}
