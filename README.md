# About

Translation file for [Nomad Sculpt](https://nomadsculpt.com/).

# Testing

For iOS and Android, the file should be named `debug.strings`.
For the web version, it simply needs to end with `.strings`.

- iOS: copy the file in `Nomad/debug.strings` and restart the app
- Android: copy the file in `Android/data/com.stephaneginier.nomad/files/debug.strings` and restart the app
- For the [Web version](https://nomadsculpt.com/demo/), simply drag & drop the file in the page

# Documentation

Should be straightforward.
Empty strings `""` will use english as a fallback.

# Note
Emojis not supported.

# Traditional Chinese

```
opencc -i "locales/chinese-simplified.strings" -o "locales/chinese-traditional.strings" -c s2twp
opencc -i "description/chinese-simplified.txt" -o "description/chinese-traditional.txt" -c s2twp
```

<!-- 
# Integration

```
crowdin download && ./../../build/nomad a
crowdin upload
crowdin upload translations --auto-approve-imported --import-eq-suggestions
```
 -->