# About

Translation file for [Nomad Sculpt](https://nomadsculpt.com/]).

Handled by https://localazy.com/p/nomad

# Testing

For iOS and Android, the file should be named `debug.strings`.
For the web version, it simply needs to end with `.strings`.

- iOS: copy the file in `Nomad/debug.strings` and restart the app
- Android: copy the file in `Android/data/com.stephaneginier.nomad/files/debug.strings` and restart the app
- For the [Web version](https://stephaneginier.com/archive/nomad_demo/), simply drag n drop the file

# Documentation

Should be straightforward.
Empty strings `""` will use english as a fallback.

# Note
Emojis are not supported.

```
crowdin download && ./../../build/nomad a
crowdin upload
crowdin upload translations --auto-approve-imported --import-eq-suggestions
```

```
localazy download && ./../../build/nomad a
localazy upload
localazy upload all
```