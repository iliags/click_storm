module static;

fn add_position(_: ClickStormInterface, _: int, _: int) -> ;

fn click_at(_: ClickStormInterface, _: int, _: int, _: MouseButton) -> ;

fn click_within(_: ClickStormInterface, _: int, _: int, _: int, _: int, _: MouseButton) -> ;

fn drag_from_to(_: ClickStormInterface, _: int, _: int, _: int, _: int, _: MouseButton) -> ;

fn drag_from_to_rel(_: ClickStormInterface, _: int, _: int, _: int, _: int, _: MouseButton) -> ;

fn drag_to(_: ClickStormInterface, _: int, _: int, _: MouseButton) -> ;

fn get_screen_size(_: ClickStormInterface) -> ScreenSize;

fn move_mouse(_: ClickStormInterface, _: int, _: int) -> ;

fn new_click_storm() -> ClickStormInterface;

fn rand_bool(_: ClickStormInterface) -> bool;

fn rand_bool(_: ClickStormInterface, _: float) -> bool;

fn rand_range(_: ClickStormInterface, _: int, _: int) -> int;

fn rand_range_excl(_: ClickStormInterface, _: int, _: int) -> int;

fn set_key(_: ClickStormInterface, _: AppKeycode, _: ButtonDirection) -> ;

module AppKeycode {
const A: AppKeycode;

const Alt: AppKeycode;

const Apostrophe: AppKeycode;

const B: AppKeycode;

const BackSlash: AppKeycode;

const Backspace: AppKeycode;

const C: AppKeycode;

const CapsLock: AppKeycode;

const Comma: AppKeycode;

const Command: AppKeycode;

const Control: AppKeycode;

const D: AppKeycode;

const Delete: AppKeycode;

const Dot: AppKeycode;

const Down: AppKeycode;

const E: AppKeycode;

const End: AppKeycode;

const Enter: AppKeycode;

const Equal: AppKeycode;

const Escape: AppKeycode;

const F: AppKeycode;

const F1: AppKeycode;

const F10: AppKeycode;

const F11: AppKeycode;

const F12: AppKeycode;

const F13: AppKeycode;

const F14: AppKeycode;

const F15: AppKeycode;

const F16: AppKeycode;

const F17: AppKeycode;

const F18: AppKeycode;

const F19: AppKeycode;

const F2: AppKeycode;

const F20: AppKeycode;

const F3: AppKeycode;

const F4: AppKeycode;

const F5: AppKeycode;

const F6: AppKeycode;

const F7: AppKeycode;

const F8: AppKeycode;

const F9: AppKeycode;

const G: AppKeycode;

const Grave: AppKeycode;

const H: AppKeycode;

const Home: AppKeycode;

const I: AppKeycode;

const Insert: AppKeycode;

const J: AppKeycode;

const K: AppKeycode;

const Key0: AppKeycode;

const Key1: AppKeycode;

const Key2: AppKeycode;

const Key3: AppKeycode;

const Key4: AppKeycode;

const Key5: AppKeycode;

const Key6: AppKeycode;

const Key7: AppKeycode;

const Key8: AppKeycode;

const Key9: AppKeycode;

const L: AppKeycode;

const LAlt: AppKeycode;

const LControl: AppKeycode;

const LMeta: AppKeycode;

const LOption: AppKeycode;

const LShift: AppKeycode;

const Left: AppKeycode;

const LeftBracket: AppKeycode;

const M: AppKeycode;

const Meta: AppKeycode;

const Minus: AppKeycode;

const N: AppKeycode;

const Numpad0: AppKeycode;

const Numpad1: AppKeycode;

const Numpad2: AppKeycode;

const Numpad3: AppKeycode;

const Numpad4: AppKeycode;

const Numpad5: AppKeycode;

const Numpad6: AppKeycode;

const Numpad7: AppKeycode;

const Numpad8: AppKeycode;

const Numpad9: AppKeycode;

const NumpadAdd: AppKeycode;

const NumpadDecimal: AppKeycode;

const NumpadDivide: AppKeycode;

const NumpadEnter: AppKeycode;

const NumpadEquals: AppKeycode;

const NumpadMultiply: AppKeycode;

const NumpadSubtract: AppKeycode;

const O: AppKeycode;

const Option: AppKeycode;

const P: AppKeycode;

const PageDown: AppKeycode;

const PageUp: AppKeycode;

const Q: AppKeycode;

const R: AppKeycode;

const RAlt: AppKeycode;

const RControl: AppKeycode;

const RMeta: AppKeycode;

const ROption: AppKeycode;

const RShift: AppKeycode;

const Right: AppKeycode;

const RightBracket: AppKeycode;

const S: AppKeycode;

const Semicolon: AppKeycode;

const Shift: AppKeycode;

const Slash: AppKeycode;

const Space: AppKeycode;

const T: AppKeycode;

const Tab: AppKeycode;

const U: AppKeycode;

const Up: AppKeycode;

const V: AppKeycode;

const W: AppKeycode;

const X: AppKeycode;

const Y: AppKeycode;

const Z: AppKeycode;

op !=(AppKeycode, AppKeycode) -> bool;

op ==(AppKeycode, AppKeycode) -> bool;

/// Return the current variant of `MouseButton`.
fn get enum_type(value: AppKeycode) -> String;

fn get value(_: AppKeycode) -> ?;

fn to_debug(value: AppKeycode) -> String;

fn to_string(value: AppKeycode) -> String;
}

module ButtonDirection {
const Click: ButtonDirection;

const Press: ButtonDirection;

const Release: ButtonDirection;

op !=(ButtonDirection, ButtonDirection) -> bool;

op ==(ButtonDirection, ButtonDirection) -> bool;

/// Return the current variant of `ButtonDirection`.
fn get enum_type(value: ButtonDirection) -> String;

/// Return the value of `ButtonDirection`.
fn get value(_: ButtonDirection) -> ?;

fn to_debug(value: ButtonDirection) -> String;

fn to_string(value: ButtonDirection) -> String;
}

module MouseButton {
const Left: MouseButton;

const Middle: MouseButton;

const Right: MouseButton;

const ScrollDown: MouseButton;

const ScrollLeft: MouseButton;

const ScrollRight: MouseButton;

const ScrollUp: MouseButton;

op !=(MouseButton, MouseButton) -> bool;

op ==(MouseButton, MouseButton) -> bool;

/// Return the current variant of `MouseButton`.
fn get enum_type(value: MouseButton) -> String;

/// Return the value of `MouseButton`.
fn get value(_: MouseButton) -> ?;

fn to_debug(value: MouseButton) -> String;

fn to_string(value: MouseButton) -> String;
}

module ScreenSize {
/// Get the center coordinates of the screen
fn center(object: ScreenSize) -> ScreenSize;

/// Get the height of the screen
fn get height(object: ScreenSize) -> int;

/// Get the width of the screen
fn get width(object: ScreenSize) -> int;

fn to_debug(object: ScreenSize) -> String;

fn to_string(object: ScreenSize) -> String;
}


