CLI DX

- Minimal WAV pipeline

  > End-to-end pipeline, audible audio
  - Command:
    $ ffmpreg -i input.wav -o output.wav
  - Output: WAV written, playable immediately

- Frame inspection / Media info

  > Show frame details, minimal ffprobe
  - Command:
    $ ffmpreg -i input.wav --show
  - Output:
    Frame 0: pts=0, samples=1024, channels=2, rate=44100
    Frame 1: pts=1024, samples=1024, channels=2, rate=44100

- Basic transform

  > Apply simple transform (gain)
  - Command:
    $ ffmpreg -i input.wav -o output.wav --apply gain=2.0
  - Output: audio amplified x2

- Multi-file / batch

  > Process multiple files in one command
  - Command:
    $ ffmpreg --input folder/\*.wav --output out/
  - Output: each file processed into out/

- More containers

  > Support raw video (Y4M)
  - Command:
    $ ffmpreg -i input.y4m -o output.y4m
  - Output: decoded/encoded video frames

- More codecs

  > Encode/decode multiple codecs
  - Command:
    $ ffmpreg -i input.adpcm -o output.wav --codec adpcm
    $ ffmpreg -i input.wav -o output.adpcm --codec adpcm
  - Output: roundtrip decode/encode working

- Chained filters
  > Apply multiple transforms in sequence
  - Command:
    $ ffmpreg -i input.wav -o output.wav --apply gain=2.0 --apply normalize
  - Output: audio amplified and normalized

```rs
INPUT      otonoke.mp3      3:15    [48kHz stereo 16-bit 132 kb/s]   3.7 MiB
└─ track=0  audio  mp3        fltp

INPUT      sparkle.wav      0:42    [44.1kHz mono 16-bit 1411 kb/s]  4.5 MiB
└─ track=0  audio  pcm_s16le

ENCODE     otonoke.flac    [48kHz stereo 24-bit]   flac
└─ track=0  audio  s32
└─ transforms: resample=48kHz

ENCODE     sparkle.flac    [44.1kHz mono 24-bit]   flac
└─ track=0  audio  s32
└─ transforms: resample=44.1kHz

DONE       otonoke.flac    37.4 MiB   1566 kb/s   speed=344x   overhead=0.02%
DONE       sparkle.flac    4.5 MiB    1411 kb/s   speed=360x   overhead=0.01%
```
