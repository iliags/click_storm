# Changelog

All notable changes to this project will be documented in this file.

## 0.2.4

* Update dependencies
* Added clamp value toggle as a workaround for [#12](https://github.com/iliags/click_storm/issues/12)

## 0.2.3

* Update egui to 0.29
* Fix [#12](https://github.com/iliags/click_storm/issues/12)

## 0.2.2

* Basic script editor with log output
  * VSCode is still recommended for larger scripts
* Script execution
* Script integration modules moved into scripting crate
* Added warning about using 0ms in Clicker (don't do it, things misbehave)

## 0.2.1

* Fix Mac build

## 0.2.0

* Embedded [Rhai](https://rhai.rs/) scripting engine, scripting features will be enabled in a later version
  * Scripting related features are locked behind the `scripting` feature flag when building
  * The API is still in heavy development, it will likely change between versions
* Refactored code into workspace with crates
  * `cs_codegen`: CLI tool that generates the API definitions (single file) in the current folder
  * `cs_hal`: Hardware abstraction layer
  * `cs_scripting`: Scripting engine access point and API definition
* Refactored worker thread to use one thread instead of two.
* Disabled minimize because the app stops receiving input when that happens
* Re-enabled maximize

## 0.1.7

* Disabled always on top
* Disabled maximize

## 0.1.6

* Translations for French, German, and Spanish (Spain)
* UI cleanup

## 0.1.5

* Click variation
* Disable start button while running
* Disable center panel while running
* Turbo clicking added
* Turbo clicking set as the default mode
* Custom hotkey setting
* Use icon for settings menu button

## 0.1.4

* Workflow file renamed from `deploy.yml` to `release.yml`
* Moved some UI variables out of app settings to reduce memory usage when sending data to background thread

## 0.1.3

* Added changelog

## 0.1.2

* Update dependencies
* Updated icons
* App icon for Windows executables

## 0.1.1

* CI/CD fixes

## 0.1.0

* Everything
