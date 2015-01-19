# winapi-rs [![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/retep998/winapi-rs) #

This crate provides types and constants for WinAPI FFI bindings. They are gathered by hand using the very latest official SDK from Microsoft.

If this crate is missing something you need, feel free to create an issue, open a pull request, or contact me via IRC (WindowsBunny on #rust-gamedev on irc.mozilla.org).

This crate is currently transitioning away from using Cargo features to having separate crates handle function bindings for each library. Available libraries:
* [advapi32](https://github.com/retep998/advapi32-sys)
* [kernel32](https://github.com/retep998/kernel32-sys)

[Documentation](https://retep998.github.io/winapi-rs/winapi/)
