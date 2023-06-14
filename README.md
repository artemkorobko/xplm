# X-Plane plugin APIs for Rust

## Purpose

This library provides a convenient interface for X-Plane plugin development in the Rust programming language for all platforms.

We use the [xplm-sys](https://github.com/artemkorobko/xplm-sys), any plugin created with this library
supports X-Plane version defined in this library.

# Features

- [X] [XPLMPlugin](https://developer.x-plane.com/sdk/XPLMPlugin) (except XPLMEnumerateFeatures, use XPLMHasFeature instead)
- [] [XPLMMenus](https://developer.x-plane.com/sdk/XPLMMenus)
- [] [XPLMUtilities](https://developer.x-plane.com/sdk/XPLMUtilities) (except XPLMExtractFileAndPath and XPLMGetDirectoryContents, use Rust equivalents instead)
