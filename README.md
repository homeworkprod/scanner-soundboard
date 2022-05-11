# Scanner Soundboard

Reads codes via RFID or 1D/2D barcode USB scanners and plays soundfiles
mapped to them.

The input device is grabbed exlusively so that scanned codes will be
passed to the program regardless of what program/window currently has
focus.

I originally developed this to play insider jokes as custom sounds
(generated via text-to-speech engines) during regular internal evenings
of [Among Us](https://www.innersloth.com/games/among-us/) games. The
sounds are triggered by placing 3D-printed Among Us figurines (glued to
coin-size RFID tags) on a cheap (~12 €) USB RFID reader, itself covered
by a 3D-printed plan of a map from the game.


## Usage

1. Have a bunch of sound files.

2. Have a bunch of codes to trigger the sounds. Those codes can come
   from RFID tags (10-digit strings seem to be common) or whatever you
   can fit in a 1D barcode or matrix/2D barcode (Aztec Code, Data
   Matrix, QR code, etc.). Anything your scanner supports.

3. Specify the path of the sound files and map the codes to sound
   filenames in a configuration file (see `config-example.toml` for an
   example).

4. Find out where your scanner is available as a device. `sudo lsinput`
   and `sudo dmesg | tail` can help you here. Note that the path can
   change over time, depending on the order devices are connected.

5. Run the program, pointing to the configuration file and input device:

   ```sh
   $ scanner-soundboard -c config.toml -i /dev/input/event23
   ```


## Sound Formats

Ogg Vorbis and MP3 are supported out of the box. However, the employed
audio playback library ([rodio](https://github.com/RustAudio/rodio))
also supports FLAC, WAV, MP4 and AAC, but those have to be enabled as
features in `Cargo.toml` and require recompilation of the program.


## License

Scanner Soundboard is licensed under the MIT license.
