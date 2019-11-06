# Rusty-craft

This is (an attempt at) a Minecraft-like, written in Rust.

It is my first attempt at:
- Rust
- Programming a game engine
- non-browser GUIs
So there are likely to be ineffeciencies and non-standard methods of working.

## Pre-Build steps
You need to make sure you have SDL development libraries available for the `sdl2` crate. There's more information available [in the Crate repositories README](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries), but the basic information is:
### POSIX (linux or MacOS)
Use your app manager (apt-get, homebrew, pacman, etc) to install `sld2` or `libsdl2-dev`.

### Windows
You should download the relevant development libraries from http://www.libsdl.org/download-2.0.php
and add the appropriate files into folders as shown below:
```
SDL2-devel-2.0.x-mingw.tar.gz\SDL2-2.0.x\i686-w64-mingw32\bin 		-> 	gnu-mingw\dll\32
SDL2-devel-2.0.x-mingw.tar.gz\SDL2-2.0.x\x86_64-w64-mingw32\bin 	-> 	gnu-mingw\dll\64
SDL2-devel-2.0.x-mingw.tar.gz\SDL2-2.0.x\i686-w64-mingw32\lib 		-> 	gnu-mingw\lib\32
SDL2-devel-2.0.x-mingw.tar.gz\SDL2-2.0.x\x86_64-w64-mingw32\lib 	-> 	gnu-mingw\lib\64
SDL2-devel-2.0.8-VC.zip\SDL2-2.0.x\lib\x86\*.dll	 		-> 	msvc\dll\32
SDL2-devel-2.0.8-VC.zip\SDL2-2.0.x\lib\x64\*.dll 			-> 	msvc\dll\64
SDL2-devel-2.0.8-VC.zip\SDL2-2.0.x\lib\x86\*.lib	 		-> 	msvc\lib\32
SDL2-devel-2.0.8-VC.zip\SDL2-2.0.x\lib\x64\*.lib	 		-> 	msvc\lib\64
```
For example: If you are building for a 64-bit windows system using the Visual code build chain, you should download the `VC` libraries, and create folders at the root of this repository as:
```
msvc
    \dll
        \64
    \lib
        \64
```
Then copy in the appropriate `.dll` into the `\dll\64` folder and the `.lib` files into the `\lib\64`.
