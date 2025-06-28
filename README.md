# nisabo

Current state - development

## Installation

- install Rust
- `cargo build --release`

- for debugging (using logger):
  - `RUST_LOG=info cargo run`
  - `RUST_LOG=debug cargo run`

### IMPORTANT

I set up GitHub Actions to automatically compile the app, but they don't work as
expected:
- Windows: Defender falsely flags the build as a virus (even though VirusTotal
  reports it clean).
  Additionally, the build fails due to OpenGL issues.
- Linux: Incompatibilities between different GLIBC versions cause problems, and statically linking
  with musl does not fully resolve them.
- MacOS: I have never tested it, but it is likely to have similar issues.

So, general recommendation - build the app yourself :)

## License

This project is licensed under the MIT License.

### Third-party Licenses

#### Icons under PD license

- https://www.svgrepo.com/svg/512677/plus-circle-1425
- https://www.svgrepo.com/svg/512798/save-item-1411
- https://www.svgrepo.com/svg/511409/arrow-repeat-236

#### This app uses the following third-party libraries:

- [`egui`](https://github.com/emilk/egui) — MIT OR Apache-2.0 License
  Copyright (c) 2020 Emil Ernerfeldt

`egui` is used under the terms of the MIT License.

#### Fonts

**Noto Fonts**
**Roboto Fonts**
- [`Noto Fonts`](https://fonts.google.com/)
Licensed under the SIL Open Font License, Version 1.1.
Copyright (c) Google.

**DejaVu Fonts**
- [`DejaVu Fonts`](https://dejavu-fonts.github.io/)
Licensed under Public domain.
Copyright (c) Bitstream Vera Fonts.
Copyright (c) Arev Fonts by Tavmjong Bah.
