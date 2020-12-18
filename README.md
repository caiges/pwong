# Building

## Linux and OSX

Install SDL libraries:

- sdl2
- sdl2_gfx
- sdl2_mixer
- sdl2_ttf

## Windows

Install Visual Studio build tools and ensure cmake is installed and on the user's path. It should be something like: `C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin`.

A build script, `build.rs` handles placing dependent libraries in the root of this project which is where cargo will look for dynamic libraries when running the project with `cargo run`.
