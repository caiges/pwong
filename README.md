# Building

## Linux and OSX

Install SDL libraries:

- sdl2
- sdl2_gfx
- sdl2_mixer
- sdl2_ttf

## Windows

A build script, `build.rs` handles placing dependent libraries in the root of this project which is where cargo will look for dynamic libraries when running the project with `cargo run`.
