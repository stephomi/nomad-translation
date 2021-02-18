# About

Translation file for the [Nomad Sculpt](https://nomadsculpt.com/]), available on iOS and Android.

# Testing

For iOS and Android, the file should be named `debug.rs`.
For the web version, it simply needs to end with `.rs`.

- iOS: copy the file in `Nomad/debug.rs` and restart the app
- Android: copy the file in `Android/data/com.stephaneginier.nomad/files/debug.rs` and restart the app
- For the [Web version](https://stephaneginier.com/archive/nomad_demo/), simply drag n drop the file

# Documentation

Should be straightforward.
Empty strings `""` will use english as a fallback.

# Note
It's possible that some characters might be missing (mostly for CJK ideograms).

The translation files are simply raw text, with easy parsing in mind.
The file extension is `.rs` because the syntax coloring is somewhat ok, but these are not Rush files.

