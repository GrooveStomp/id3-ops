The main idea here is that we're given a root directory containing mp3 files.
The assumed structure is like so:

```
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
      +--[album1]
         +--[01 Mp3 Title].mp3
         +--[02 Mp3 Title].mp3
         +--[03 Mp3 Title].mp3
         +--[04 Mp3 Title].mp3
         +--[05 Mp3 Title].mp3
```

We read the directory structure to construct id3 tags for the given mp3s.
The tool allows reading and writing of id3 tags.

# Dependencies
- Go toolchain
- github.com/mikkyang/id3-go

# Usage
```
./id3-ops help
./id3-ops read $MUSIC_ROOT
./id3-ops write $SPECIFIC_ALBUM_DIRECTORY
```

# Development
Clone this repo into your $GOPATH appropriately.
```
cd /path/to/this/repo
```

## Installation
```
go get
```

## Building
```
go build
```
