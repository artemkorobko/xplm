# X-Plane plugin APIs for Rust

## Purpose

This library provides a convenient interface for X-Plane plugin development in the Rust programming language for all platforms.

We use the [xplm-sys](https://github.com/artemkorobko/xplm-sys), any plugin created with this library
supports X-Plane version defined in this library.

# Features

- [ ] [XPLMPlugin](https://developer.x-plane.com/sdk/XPLMPlugin)
    - [X] Find Plugins.
    - [X] Enable/Disable plugins.
    - [X] Interplugin Messagging.
    - [ ] Plugin Features API. `XPLMEnumerateFeatures` is not yet implemented.
- [ ] [XPLMMenus](https://developer.x-plane.com/sdk/XPLMMenus)
- [X] [XPLMUtilities](https://developer.x-plane.com/sdk/XPLMUtilities)
    - [X] Full and Relative Paths.  
    - [X] X-Plane Misc except `XPLMFindSymbol`.  
    - [X] Command management.
`XPLMExtractFileAndPath` and `XPLMGetDirectoryContents` functions are not mapped to safe Rust because there is an ability use Rust equivalent functions instead.
