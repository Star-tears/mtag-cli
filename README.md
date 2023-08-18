# mtag-cli

Music Tag Organizer

Organize music for self-built media libraries, like plex, emby, jellyfin, through meta-information about the music, tag information

## Install

### Cargo

`cargo install mtag-cli`

## Usage

```bash
Usage: mtag.exe [OPTIONS] <MUSIC_FOLDER> [TARGET_FOLDER]

Arguments:
  <MUSIC_FOLDER>
  [TARGET_FOLDER]  [default: Music]

Options:
  -n, --name <NAME>
  -h, --help         Print help
  -V, --version      Print version
```

## Example

```bash
mtag ./test ./Music
```

before:

```shell
/test
	 01 - Hooked On A Feeling.mp3
     02 - Go All The Way.mp3
     03 - Spirit In The Sky.mp3
     01 - Burn.mp3
     02 - Golgotha Tenement Blues.mp3
     03 - Big Empty.mp3
     101 - In the Flesh.mp3
     102 - The Thin Ice.mp3
     201 - Hey You.mp3
     202 - Is There Anybody Out There.mp3
     01 - Shine On You Crazy Diamond (Parts I-V).m4a
     02 - Welcome to the Machine.mp3
     03 - Have a Cigar.mp3
```

after:

```
/Music
   /Various Artists
      /Guardians Of The Galaxy - Awesome Mix Vol. 1
         01 - Hooked On A Feeling.mp3
         02 - Go All The Way.mp3
         03 - Spirit In The Sky.mp3
      /The Crow - Original Motion Picture Soundtrack
         01 - Burn.mp3
         02 - Golgotha Tenement Blues.mp3
         03 - Big Empty.mp3
   /Pink Floyd
      /The Wall
         101 - In the Flesh.mp3
         102 - The Thin Ice.mp3
         201 - Hey You.mp3
         202 - Is There Anybody Out There.mp3
      /Wish You Were Here
         01 - Shine On You Crazy Diamond (Parts I-V).m4a
         02 - Welcome to the Machine.mp3
         03 - Have a Cigar.mp3
```

