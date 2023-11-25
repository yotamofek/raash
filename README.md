# raash

An attempt at [RIIR](https://www.urbandictionary.com/define.php?term=riir)-ing the native AAC encoder from [`ffmpeg`](https://ffmpeg.org/).

First, I used [`c2rust`](https://github.com/immunant/c2rust) to translate all relevant C code into Rust, and I'm in the process of re-writing it in a more Rust-y way, bit-by-bit.

## State

The resultant library can be used in lieu of the native ffmpeg AAC encoder, and produces reasonable (to my ears) AAC-encoded audio.

Most of the code is still the C code translated verbatim into Rust, and I'm pretty certain I introduced bugs and mistakes in the code I did translate.
