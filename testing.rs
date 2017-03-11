#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::{fs,io};
use std::fs::DirEntry;
use std::path::Path;
use std::ffi::OsStr;
use std::borrow::Cow;
use std::ops::Deref;

fn dir_leaves(dir: &Path) -> Box<Vec<Box<&Path>>> {
    let mut leaves = Box::new(Vec::new());
    {
        let cb = |direntry: &DirEntry| {
            let dirpath = direntry.path();
            let path_copy = Box::new(String::from(dirpath.as_path().to_str().unwrap()));
            let path = Box::new(Path::new(<Box<String> as Deref>::deref(&path_copy)));
            leaves.push(path);
        };
        visit_dirs(dir, &cb);
    }
    leaves
}

fn visit_dirs(dir: &Path, cb: &FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }

    Ok(())
}

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
