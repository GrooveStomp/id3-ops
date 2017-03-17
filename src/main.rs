extern crate glob;
extern crate id3;

use std::env;
use std::path::Path;
use glob::glob;
use id3::Tag;

fn files_in(dir: &str) -> Box<Vec<String>> {
    let mut files = Box::new(Vec::new());

    for entry in glob(dir).expect("Failed to read glob") {
        match entry {
            Ok(path) => {
                files.push(String::from(path.to_string_lossy()));
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }

    return files;
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

fn track_and_title(filename: String) -> Option<(u32, String)> {
    let filename = filename.clone();
    let mut space_splits = filename.split_whitespace();

    let track: &str;
    match space_splits.nth(0) {
        Some(t) => track = t,
        None => return None,
    }

    let title = space_splits.collect::<Vec<_>>().join(" ");

    return Some((track.parse::<u32>().unwrap(), String::from(title)));
}

fn read_tags(files: &Box<Vec<String>>) {
    for f in files.iter() {
        if let Some(tag) = Tag::read_from_path(Path::new(f)).ok() {
            let frames = tag.frames();
            println!("Tag:");
            for frame in frames {
                if let Some(text) = frame.text() {
                    println!("\t{}: {}", frame.id, text)
                }
            }
        }
    }
}

fn write_tags(files: &Box<Vec<String>>) {
    for f in files.iter() {
        let path = Path::new(f);
        match artist_album_file(path) {
            None => println!("Couldn't find song info from path."),
            Some((artist,album,file)) => {

                match track_and_title(file) {
                    None => println!(", Couldn't find track and title."),
                    Some((track, title)) => {
                        let mut tag = Tag::with_version(2);
                        match Tag::read_from_path(path) {
                            Ok(read_tag) => tag = read_tag,
                            Err(_) => println!("Couldn't read tag!"),
                        }

                        tag.set_artist(artist);
                        tag.set_album(album);
                        tag.set_track(track);
                        tag.set_title(title);

                        match tag.write_to_path(path) {
                            Ok(()) => return,
                            Err(e) => println!("{}", e),
                        }
                    },
                }
            },
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        usage(&args[0]);
        return;
    }

    let path = Path::new(&args[2]).join("**").join("*.mp3");
    let files = files_in(path.to_str().unwrap());

    if args[1] == "r" || args[1] == "read" {
        read_tags(&files);
    }
    else if args[1] == "w" || args[1] == "write" {
        write_tags(&files);
    }
    else if args[1] == "rw" || args[1] == "wr" || args[1] == "readwrite" || args[1] == "writeread" {
        write_tags(&files);
        read_tags(&files);
    }
}

fn usage(program: &str) {
    println!("Usage: {} subcommand\n", program);
    println!("subcommands:");
    println!("\tr|read");
    println!("\tw|write");
    println!("\trw|readwrite");
    println!("\twr|writeread");
}
