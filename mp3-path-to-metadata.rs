#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::path::Path;
use std::ffi::OsStr;
use std::borrow::Cow;

fn artist_album_file(path: &Path) -> Option<(String, String, String)> {
    let components = path.components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>();

    let compno = components.len();
    if compno < 3 {
        return None;
    }

    let wanted = components.iter().skip(compno - 3).collect::<Vec<_>>();

    let (artist, album, file) = (wanted[0].clone(), wanted[1].clone(), wanted[2].clone());

    return Some((String::from(artist),String::from(album),String::from(file)))
}

fn track_and_title(filename: String) -> Option<(String, String)> {
    let filename = filename.clone();
    let mut space_splits = filename.split_whitespace();

    let track: &str;
    match space_splits.nth(0) {
        Some(t) => track = t,
        None => return None,
    }

    let title = space_splits.collect::<Vec<_>>().join(" ");

    return Some((String::from(track), String::from(title)));
}

fn main() {
    let path = Path::new("/home/aaron/Music/Jack Johnson/On and On/01 Times Like These.mp3");

    println!("file stem: {:?}", path.file_stem().unwrap());
    println!("path: {:?}", path.parent().unwrap());

    match artist_album_file(path) {
        None => println!("Couldn't find song info from path."),
        Some((artist,album,file)) => {
            print!("{}, {}", artist, album);

            match track_and_title(file) {
                None => println!(", Couldn't find track and title."),
                Some((track, title)) => println!(", {}, {}", track, title),
            }
        },

    }
}
