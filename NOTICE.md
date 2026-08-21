# Third-party notices

Mightty is licensed under the GNU General Public License v3.0 or later
(see `LICENSE`). It redistributes the following third-party components,
which remain under their own licenses.

## Bundled fonts

`assets/Lilex[wght].ttf`, `assets/Lilex-Italic[wght].ttf`,
`assets/LilexNerdFontMono-Regular.ttf`, `assets/LilexNerdFontMono-Bold.ttf`,
`assets/LilexNerdFontMono-Italic.ttf`, `assets/LilexNerdFontMono-BoldItalic.ttf`

- **Lilex** - Copyright 2019 The Lilex Project Authors
  <https://github.com/mishamyrt/Lilex>
  Licensed under the SIL Open Font License, Version 1.1. Full text: `assets/OFL.txt`.
- The `LilexNerdFontMono-*` files are Lilex patched with the **Nerd Fonts**
  patcher v3.4.0 (<https://github.com/ryanoasis/nerd-fonts>). The patched fonts
  remain under the SIL Open Font License, Version 1.1.

These fonts are compiled into the binary via `include_bytes!` in
`src/ui/fonts.rs`, so the OFL notice above applies to distributed binaries as
well as to the source tree. The SIL OFL is compatible with the GPL; the fonts
are not covered by the GPL and are not "GPL-licensed" by their inclusion here.

## Rust dependencies

All direct crate dependencies are MIT and/or Apache-2.0 licensed. Run
`cargo tree` for the resolved set, or `cargo about`/`cargo deny` to regenerate
a full dependency licence report.
