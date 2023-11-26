# raash 🪇

An attempt at [RIIR](https://www.urbandictionary.com/define.php?term=riir)-ing the native AAC encoder from [`ffmpeg`](https://ffmpeg.org/).

First, I used [`c2rust`](https://github.com/immunant/c2rust) to translate all relevant C code into Rust, and I'm in the process of re-writing it in a more Rust-y way, bit-by-bit.

## State

The resultant library can be used in lieu of the native ffmpeg AAC encoder, and produces reasonable (to my ears) AAC-encoded audio.

Most of the code is still the C code translated verbatim into Rust, and I'm pretty certain I introduced bugs and mistakes in the code I did translate (I don't really know anything about audio encoding 😳).

## Usage

First:
```sh
cargo build
```

In order to build `ffmpeg` with this library instead of the native one, after cloning the `ffmpeg` repo the following changes must be changed to the configuration and build files:

* Comment out building the C obj files for the native encoder in [`libavcodec/Makefile`](https://github.com/FFmpeg/FFmpeg/blob/master/libavcodec/Makefile#L188-L195):

  ```make
  # OBJS-$(CONFIG_AAC_ENCODER)             += aacenc.o aaccoder.o aacenctab.o    \
  #                                           aacpsy.o aactab.o      \
  #                                           aacenc_is.o \
  #                                           aacenc_tns.o \
  #                                           aacenc_ltp.o \
  #                                           aacenc_pred.o \
  #                                           psymodel.o kbdwin.o \
  #                                           mpeg4audio_sample_rates.o
  ```

* Run `./configure` and then alter the resultant `ffbuild/config.mak` in order to link the `ffmpeg` binary against `raash`.
  Change the following line:
  ```text
  EXTRALIBS-avcodec=-pthread -lm -latomic
  ```
  to:
  ```text
  EXTRALIBS-avcodec=-pthread -lm -latomic -L/path/to/raash/target/debug -l:libraash.a
  ```

Now you can run `make` and then try to encode a file to AAC using your newly built `ffmpeg`:
```sh
./ffmpeg -i my_audio_file.wav -f adts my_audio_file.aac
```
