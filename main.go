package "main"

// #include<id3/tag.h>

import(
	"C"
	"fmt"

	"github.com/bogem/id3v2"
)

/*
The main idea here is that we're given a root directory containing mp3 files.
The assumed structure is like so:

+--[Root Dir]
   +--[artist1]
   |  +--[album1]
   |  |  +--[01 Mp3 Title].mp3
   |  |  +--[02 Mp3 Title].mp3
   |  |  +--[03 Mp3 Title].mp3
   |  |  +--[04 Mp3 Title].mp3
   |  |  +--[05 Mp3 Title].mp3
   |  +--[album2]
   |     +--[01 Mp3 Title].mp3
   |     +--[02 Mp3 Title].mp3
   |     +--[03 Mp3 Title].mp3
   |     +--[04 Mp3 Title].mp3
   |     +--[05 Mp3 Title].mp3
   +--[artist2]
   |  +--[album1]
   |     +--[01 Mp3 Title].mp3
   |     +--[02 Mp3 Title].mp3
   |     +--[03 Mp3 Title].mp3
   |     +--[04 Mp3 Title].mp3
   |     +--[05 Mp3 Title].mp3
   +--[artist3]
      +--[album1]
         +--[01 Mp3 Title].mp3
         +--[02 Mp3 Title].mp3
         +--[03 Mp3 Title].mp3
         +--[04 Mp3 Title].mp3
         +--[05 Mp3 Title].mp3
*/
func main() {

	tag, err := id3v2.Open("...")
	if err != nil {
		log.Fatal("...", err)
	}
	defer tag.Close()

	tag.SetArtist("...")
	tag.SetTitle("...")
	tag.SetAlbum("...")


	fmt.Printf("Hi\n")
}
