# winapi-rs [![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/retep998/winapi-rs) #

This crate provides types and constants for WinAPI FFI bindings. They are gathered by hand using the very latest official SDK from Microsoft. I aim to replace all existing Windows FFI in other crates with this set of crates through the "[Embrace, extend, and extinguish](http://en.wikipedia.org/wiki/Embrace,_extend_and_extinguish)" technique.

If this crate is missing something you need, feel free to create an issue, open a pull request, or contact me via IRC (WindowsBunny on #rust-gamedev on irc.mozilla.org).

Bindings to library functions are available in separate crates:
* [advapi32](https://github.com/retep998/advapi32-sys)
* [gdi32](https://github.com/retep998/gdi32-sys)
* [kernel32](https://github.com/retep998/kernel32-sys)
* [ole32](https://github.com/retep998/ole32-sys)
* [shell32](https://github.com/retep998/shell32-sys)
* [user32](https://github.com/retep998/user32-sys)
* [winmm](https://github.com/retep998/winmm-sys)

[Documentation](https://retep998.github.io/winapi-rs/winapi/)
