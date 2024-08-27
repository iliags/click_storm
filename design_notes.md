# Design Notes

Misc notes for development.

## Rhai

Rhai is a scripting language which can be embedded inside a Rust program. The idea is to allow users to write their own input scripts in addition to the default behavior. Ideally the scripting feature can be disabled to allow the base implementation to remain relatively untouched.

## Rhai Implementation

Implementation notes:

- The click loop will be re-implemented in Rhai script
- The UI will behave the same except push the settings to the Rhai script instead of a custom thread
  - The internal scripts will be included as bytes to ensure they are not tampered with
  - See Scope and push_constant
- Users can write their own scripts to control input
- There will need to be more type conversions than initially expected.
