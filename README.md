# X-Plane plugin APIs for Rust

## Purpose

This library provides a convenient interface for X-Plane plugin development in the Rust programming language for all platforms.

It uses the [xplm-sys](https://github.com/artemkorobko/xplm-sys), any plugin created with this library
supports X-Plane version defined in this library.

# API Features

- [ ] [XPLMCamera](https://developer.x-plane.com/sdk/XPLMCamera)
- [ ] [XPLMDataAccess](https://developer.x-plane.com/sdk/XPLMDataAccess)
     - [X] Reading and writing data.
     - [X] Data accessors.
     - [ ] Publishing plugin's data.
     - [ ] Sharing data between multiple plugins.
- [ ] [XPLMDefs](https://developer.x-plane.com/sdk/XPLMDefs)
- [ ] [XPLMDisplay](https://developer.x-plane.com/sdk/XPLMDisplay)
    - [ ] Drawing callbacks.
    - [ ] Avionics API.
    - [X] Window API except `XPLMGetAllMonitorBoundsGlobal`, `XPLMGetAllMonitorBoundsOS`, `XPLMGetWindowGeometryVR`, `XPLMSetWindowGeometryVR`, `XPLMWindowIsInVR`, `XPLMGetWindowRefCon`, `XPLMSetWindowRefCon`.
    - [ ] Key Sniffers.
    - [ ] Hot Keys.
- [ ] [XPLMGraphics](https://developer.x-plane.com/sdk/XPLMGraphics)
    - Except `XPLMGenerateTextureNumbers`, `XPLMGetTexture`.
- [ ] [XPLMInstance](https://developer.x-plane.com/sdk/XPLMInstance)
- [X] [XPLMMenus](https://developer.x-plane.com/sdk/XPLMMenus)
- [ ] [XPLMMap](https://developer.x-plane.com/sdk/XPLMMap)
- [ ] [XPLMNavigation](https://developer.x-plane.com/sdk/XPLMNavigation)
- [ ] [XPLMPlanes](https://developer.x-plane.com/sdk/XPLMPlanes)
- [ ] [XPLMPlugin](https://developer.x-plane.com/sdk/XPLMPlugin)
    - [X] Find Plugins.
    - [X] Enable/Disable plugins.
    - [X] Interplugin Messagging.
    - [ ] Plugin Features API. `XPLMEnumerateFeatures` is not yet implemented.
- [X] [XPLMProcessing](https://developer.x-plane.com/sdk/XPLMProcessing)
    - [X] XPLMGetElapsedTime
    - [X] XPLMGetCycleNumber
    - [X] XPLMCreateFlightLoop
    - [X] XPLMDestroyFlightLoop
    - [X] XPLMScheduleFlightLoop
  
  The following API calls will not be implemented `XPLMRegisterFlightLoopCallback`, `XPLMUnregisterFlightLoopCallback`, `XPLMSetFlightLoopCallbackInterval`.
- [ ] [XPLMScenery](https://developer.x-plane.com/sdk/XPLMScenery)
- [ ] [XPLMSound](https://developer.x-plane.com/sdk/XPLMSound)
- [X] [XPLMUtilities](https://developer.x-plane.com/sdk/XPLMUtilities)
    - [X] Full and Relative Paths.  
    - [X] X-Plane Misc except `XPLMFindSymbol`.  
    - [X] Command management.
`XPLMExtractFileAndPath` and `XPLMGetDirectoryContents` functions are not mapped to safe Rust because there is an ability use Rust equivalent functions instead.
- [ ] [XPLMWeather](https://developer.x-plane.com/sdk/XPLMWeather)
- [ ] [XPStandardWidgets](https://developer.x-plane.com/sdk/XPStandardWidgets)
- [ ] [XPUIGraphics](https://developer.x-plane.com/sdk/XPUIGraphics)
- [ ] [XPUIGraphics](https://developer.x-plane.com/sdk/XPUIGraphics)
- [ ] [XPWidgetUtils](https://developer.x-plane.com/sdk/XPWidgetUtils)
- [ ] [XPWidgets](https://developer.x-plane.com/sdk/XPWidgets)

# Functional Features

- [X] Plugin initialization usin `xplm::register_plugin!` macro.
- [X] Logging using `xplm::info!`, `xplm::warn!` and `xplm::error!` macroses.
