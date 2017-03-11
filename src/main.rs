//#![allow(unused_imports)]
#![allow(unused_must_use)]
extern crate id3;

use id3::Tag;
use std::{fs,env};
use std::ffi::OsStr;
use std::path::Path;
use std::borrow::Cow;
use std::io::Error;

fn print_tag(tag: &Tag) {
    let frames = tag.frames();
    println!("Tag:");
    for frame in frames {
        println!("\t{}", frame.text().unwrap());
    }
}

fn read_tag(path: &Path, _: &mut Tag) -> Result<(),String> {
    match Tag::read_from_path(path) {
        Ok(tag) => {
            print_tag(&tag);
            return Ok(());
        },
        Err(err) => {
            return Err(String::from(err.description));
        },
    }
}

fn write_tag(path: &Path, tag: &mut Tag) -> Result<(),String> {
    // Should read existing tag first, then update with new info.
    let file_name = Cow::from(path.file_name().unwrap().to_str().unwrap());

    let no_ext = file_name.split('.').collect::<Vec<_>>()[0];

    let mut no_spaces = no_ext.split_whitespace();

    let track = no_spaces.nth(0).unwrap();
    let title = no_spaces.collect::<Vec<_>>().join(" ");

    tag.set_track(track.parse::<u32>().unwrap());
    tag.set_title(title);

    match tag.write_to_path(path) {
        Ok(()) => return Ok(()),
        Err(err) => {
            return Err(String::from(err.description));
        }
    }
}

fn parse_directory(music_dir: &OsStr, func: &Fn(&Path, &mut Tag) -> Result<(),String>) -> Result<(),Error> {

    // TODO: Instead of recursing like this, just find all files and chop up the path name.

    let mut id3v2_2 = Tag::with_version(2);

    for artist in try!(fs::read_dir(music_dir)) {
        let artist = try!(artist);
        let file_type = try!(artist.file_type());
        if file_type.is_dir() {
            id3v2_2.set_artist(artist.file_name().into_string().unwrap());

            for album in try!(fs::read_dir(artist.path())) {
                let album = try!(album);
                let file_type = try!(album.file_type());
                if file_type.is_dir() {
                    id3v2_2.set_album(album.file_name().into_string().unwrap());

                    for song in try!(fs::read_dir(album.path())) {
                        let song = try!(song);
                        let file_type = try!(song.file_type());
                        if file_type.is_file() {

                            func(&song.path(), &mut id3v2_2);
                        }}}}}}

    Ok(())
}

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        usage(&args[0]);
        return;
    }

    let path = OsStr::new("/home/aaron/Music/test/");

    if args[1] == "r" || args[1] == "read" {
        parse_directory(path, &read_tag);
    }
    else if args[1] == "w" || args[1] == "write" {
        parse_directory(path, &write_tag);
    }
    else if args[1] == "rw" || args[1] == "wr" || args[1] == "readwrite" || args[1] == "writeread" {
        let closure = |path: &Path, tag: &mut Tag| {
            match write_tag(path, tag) {
                Ok(())   => return read_tag(path, tag),
                Err(err) => return Err(err),
            }
        };

        parse_directory(path, &closure);
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
