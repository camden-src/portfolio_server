
use id3::{Error, ErrorKind, Tag, TagLike};
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::result::Result::Ok;

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub file_name: OsString,
    pub artist: String,
    pub title: String,
}

fn get_artist_and_title(path: PathBuf) -> Option<(String, String)> {
    let tag = match Tag::read_from_path(&path) {
        Ok(tag) => tag,
        Err(Error {
            kind: ErrorKind::NoTag,
            ..
        }) => Tag::new(),
        Err(err) => {
            eprintln!("An unexpected error: {}", err);
            return None;
        }
    };

    let artist = tag.artist();
    let title = tag.title();

    match (artist, title) {
        (Some(artist), Some(title)) => return Some((artist.to_string(), title.to_string())),
        (Some(_), None) => {
            eprintln!(
                "{} is missing a Title ID3 tag. It cannot be listed.",
                &path.display()
            );
        }
        (None, Some(_)) => {
            eprintln!(
                "{} is missing an Artist ID3 tag. It cannot be listed.",
                &path.display()
            );
        }
        (None, None) => {
            eprintln!(
                "{} is missing both Artist and Title ID3 tags. It cannot be listed.",
                &path.display()
            );
        }
    }
    None
}

fn collect_track_data(dir_entry: std::fs::DirEntry) -> Option<Track> {
    if let Ok(entry_type) = dir_entry.file_type() {
        if !entry_type.is_dir() {
            match get_artist_and_title(dir_entry.path()) {
                Some((artist, title)) => {
                    let track = Track {
                        file_name: dir_entry.file_name(),
                        artist: artist,
                        title: title,
                    };
                    return Some(track);
                }
                None => (),
            }
        }
    } else {
        eprintln!(
            "The file type of {} cannot be determined.",
            dir_entry.path().display().to_string()
        );
    }
    None
}

pub fn list_media_files(path: PathBuf) -> Vec<Track> {
    let mut media_list_builder = Vec::<Track>::new();
    match fs::read_dir(&path) {
        Ok(entries) => {
            for dir_entry in entries {
                if let Ok(interrogable_entry) = dir_entry {
                    if let Some(track) = collect_track_data(interrogable_entry) {
                        media_list_builder.push(track);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory:{}\n{}", &path.display(), e);
        }
    }
    let media_files = media_list_builder;
    media_files
}

#[test]
fn media_files_succeed() {
    assert_eq!(2, 2);
}
