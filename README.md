# About
Translation file for [Nomad Sculpt](https://nomadsculpt.com/).

# Testing
For iOS and Android, the file should be named `debug.strings`.
For the web version, it simply needs to end with `.strings`.

- iOS: copy the file in `Nomad/debug.strings` and restart the app
- Android: copy the file in `Android/data/com.stephaneginier.nomad/files/debug.strings` and restart the app
- For the [Web version](https://nomadsculpt.com/demo/), simply drag & drop the file in the page

# Documentation
Empty strings `""` will use english as a fallback.

# Note
Emojis are not supported.
Language specific comments starts with `// NOTE:` or `// TODO:`.

<!-- 
# Integration
```
crowdin upload && crowdin upload translations --auto-approve-imported --import-eq-suggestions
crowdin pre-translate --method ai --ai-prompt=130
crowdin download
opencc -i "locales/simplified-chinese.strings" -o "locales/traditional-chinese.strings" -c s2twp
```
 -->
 