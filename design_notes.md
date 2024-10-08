# Design Notes

Misc notes for development.

## Next Release TODO

- Include the example script in the program
- Implement an unsaved script check

## Rhai

Rhai is a scripting language which can be embedded inside a Rust program. The idea is to allow users to write their own input scripts in addition to the default behavior. Ideally the scripting feature can be disabled to allow the base implementation to remain relatively untouched.

### Potential Future Work

If Rhai needs to have extensive modification, it will need to have a formal grammar and internal IDE features exposed as Rust crates. The biggest potential requirement would be implementing a bytecode because it currently uses AST interpretation.

Unreal uses a stack-based VM (so does Rhai if I understand it correctly), but register VM's provide much better performance. Since Rhai doesn't have a GC, there are some implementation limitations, but that's fine. Perhaps a `no-gc` mode can be implemented on top of whatever the final result is.

## Record/Replay

Recording and replaying is done via tracking the user input and creating a Rhai script which can be exported for reuse. The recording feature should try to mimic the behavior in as few lines of code as possible to ensure the scripts aren't bloated.

## Feature Ideas

- Script file watcher (for external editing)
- Hot reload
- Visual scripting

Copied from SCAR Divi, will update later.

- [X] Use your Keyboard and Mouse to do things for you (Mouse & Keyboard functions)
- [ ] Find images/colors on your computer screen (Bitmap/Color/dtm/etc functions)
- [ ] Various Conversions for Types and Colors (Conversion functions)
- [ ] Internet, TCP, HTTP, and proxy (Internet functions)
- [ ] Hex, & Base64 Encoding (Encoding functions)
- [ ] Clipboard copy & paste (Clipboard functions)
  - I don't like the idea of OS clipboard access, it might be better to create a custom clipboard type which drops data after execution
- [ ] Time/Date/etc (Time functions)
- [ ] Rewrite/Create/Delete Files/Folders (File functions)
- [ ] Regex matching (Regex functions)
- [ ] Command line access (See Command Line parameters)
- [ ] Debugging (Debug functions)
- [ ] Grab colors/bitmaps from the screen with a tool

Much of this is already done in VSCode but it would need re-implementation in a custom code editor

- [ ] Auto completion of code
- [ ] Code hints
- [ ] Print your code
- [ ] Step by Step debugging has variable list
- [ ] go to line numbers, change highlighted colors
