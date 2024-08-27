# Design Notes

Misc notes for development.

## Rhai

Rhai is a scripting language which can be embedded inside a Rust program. The idea is to allow users to write their own input scripts in addition to the default behavior. Ideally the scripting feature can be disabled to allow the base implementation to remain relatively untouched.

## Rhai Implementation

Implementation notes:

- The click loop will be re-implemented in Rhai script for benchmarking (to see if it holds up)
- The UI will behave the same except push the settings to the Rhai script instead of a custom thread
  - The internal scripts will be included as bytes to ensure they are not tampered with
  - See Scope and push_constant
- Users can write their own scripts to control input
  - Initially, they will need to use VSCode but a script editor would be implemented.
- There will need to be more type conversions than initially expected.
- Official releases will continue to be without scripting until it's ready, the restructured code can still be released.
- When scripting is ready, the program will have a toggle to change between "classic" and "new" modes.
- A scripting terminal will be added for output

## Record/Replay

Recording and replaying is done via tracking the user input and creating a Rhai script which can be exported for reuse. The recording feature should try to mimic the behavior in as few lines of code as possible to ensure the scripts aren't bloated.

## API (WIP)

Tentative API into native rust functions, optional params use the function overloading feature. Coordinates are absolute unless noted otherwise.

Expected:

- screen_size: returns the screen size X/Y in pixels (maybe add QoL functions like screen center)
- click at: clicks (down up) at an X/Y position optionally using a button and/or a modifier key
- click in range: clicks (down up) within a from/to range in X/Y optionally using a button and/or a modifier key
- move to: instantly moves to a location, if a duration is provided interpolates from the current position to the target position over time
- drag: holds the mouse button down, moves, and releases when at the target location, optional duration
- add: adds the input X/Y to the current X/Y, optional duration for lerping
- wait: waits for N amount of time in milliseconds
- wait duration: waits for N amount of time in any duration (hours, minutes, seconds, milliseconds)
- key press: performs a key press (down up) or key chord
- key down: Holds a key (or chord) down
- key up: releases key (or chord) if they are down
- is key down: checks if a key is down
- is key up: checks if a key is up

## Feature Ideas

Copied from SCAR Divi, will update later. Most of the low-level or OS specific functionality will not be implemented.

- Use your Keyboard and Mouse to do things for you (Mouse & Keyboard functions)
- Find images/colors on your computer screen (Bitmap/Color/dtm/etc functions)
- It has a Window API for Finding/Interacting with Windows/Programs (Window/Client functions)
- Text/font (GetTextAt/load font functions) (not documented in wiki yet)
- Integer/Boolean/Extended (decimal)/Box/Point/Colors (like clBlack)/other Types (type functions) (Arrays with these types, functions)
- Strings (String functions)
- Interact with SQLite3 databases (Database functions)
- Math (Math functions)
- ZLib Compression (Compression functions)
- Supports the use of Forms (form functions)
- MD5 Hashing (Hashing functions)
- Various Conversions for Types and Colors (Conversion functions)
- Internet, TCP, HTTP, and proxy (Internet functions)
- Hex, & Base64 Encoding (Encoding functions)
- Clipboard copy & paste (Clipboard functions)
- Time/Date/etc (Time functions)
- INI files (INI functions)
- Rewrite/Create/Delete Files/Folders (File functions)
- Regex matching (Regex functions)
- Some Sound usage (Sound functions)
- Settings (Not SCAR Divi's settings) (Settings functions)
- Command line access (See Command Line parameters)
- System (Not much) (System functions)
- Debugging (Debug functions)

There is still a lot of undocumented/not up to date things SCAR Divi can do!

- Auto completion of code with (CTRL+Spacebar)
- Code hints
- Functions to minimize, set as top, minimize to tray, and others for SCAR Divi's window
- Use functions from Windows through API Calls
- Make your own plugins (Delphi, C++, other) to extend SCAR Divi's functionality
- Print your code
- Grab colors/bitmaps from the screen with a tool
- Change the client (targeted) window's title
- Save screenshots
- Has changeable hotkeys, change image to string conversion for use in your code
- Debug & messages box, color history, function list, and target client information
- Form & DTM Editor
- Find & replace code
- Step by Step debugging has variable list
- Up to date coordinates in the bottom left hand corner based on the client (targeted) window
- go to line numbers, change highlighted colors
- Updating
- Built in Firewall
