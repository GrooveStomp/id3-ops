package main

import (
	id3 "github.com/mikkyang/id3-go"
	id3v2 "github.com/mikkyang/id3-go/v2"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

var programName string

func main() {
	programName = os.Args[0]
	if len(os.Args) < 2 {
		notEnoughArguments()
	}

	op := os.Args[1]
	if op == "help" {
		usage()
	}

	if len(os.Args) < 3 {
		notEnoughArguments()
	}

	musicPath := os.Args[2]

	var walkFn filepath.WalkFunc
	if op == "read" {
		walkFn = read
	} else if op == "write" {
		walkFn = write
	} else {
		panic("Unknown operation")
	}

	err := filepath.Walk(musicPath, walkFn)
	if err != nil {
		panic(err)
	}
}

func notEnoughArguments() {
	fmt.Printf("Not enough arguments. Try `%v help'.\n", programName)
	os.Exit(0)
}

func usage() {
	text := `
Usage: %v SUBCOMMAND DIRECTORY
Read and write id3 data for mp3 files.
Execute SUBCOMMAND at the root filesystem tree located at DIRECTORY.

When writing id3 data, the directory structure is assumed to look like this:
└─ DIRECTORY
    └─ Artist Name
        └─ Album Name
            └─ ## Title.mp3

Where '##' is the track's position within the arrangement of songs on the album,
followed by a space, folloed by the song name.

SUBCOMMAND:

     create: Remove existing id3 data and create new id3 data according to
             the assumptions about DIRECTORY described above.

     read:   Read all existing id3 data in all mp3 files in DIRECTORY.

     help:   Show this help text.
`
	fmt.Printf(text, programName)
	os.Exit(0)
}

func read(path string, info os.FileInfo, err error) error {
	// do nothing if err.  Err signifies a problem accessing path.
	if info.IsDir() {
		return nil
	}

	f, err := id3.Open(path)
	if err != nil {
		return err
	}
	defer f.Close()

	fmt.Println()
	id3Tags := f.AllFrames()
	for _, tag := range id3Tags {
		fmt.Printf("%v %v\n", tag.Id(), tag.String())
	}

	return nil
}

func removeTags(mp3 *id3.File) {
	tag := mp3.Tagger

	frames := mp3.AllFrames()
	for _, frame := range frames {
		tag.DeleteFrames(frame.Id())
	}
}

func infoFromPath(path string) []string {
	parts := strings.Split(path, "/")
	artist, album, trackAndTitle := parts[len(parts)-3], parts[len(parts)-2], parts[len(parts)-1]
	parts = strings.Split(trackAndTitle, " ")
	track := parts[0]
	title := strings.Join(parts[1:], " ")

	ext := filepath.Ext(title)
	title = strings.Replace(title, ext, "", 1)

	return []string{artist,album,title,track}
}

func write(path string, info os.FileInfo, err error) error {
	// do nothing if err.  Err signifies a problem accessing path.
	if info.IsDir() {
		return nil
	}

	f, err := id3.Open(path)
	if err != nil {
		return err
	}
	defer f.Close()

	removeTags(f)
	mp3Info := infoFromPath(path)
	artist, album, title, track := mp3Info[0], mp3Info[1], mp3Info[2], mp3Info[3]

	f.SetArtist(artist)
	f.SetTitle(title)
	f.SetAlbum(album)

	trackFrameType := id3v2.V23FrameTypeMap["TRCK"]
	trackFrame := id3v2.NewTextFrame(trackFrameType, track)
	f.AddFrames(trackFrame)

	return nil
}
