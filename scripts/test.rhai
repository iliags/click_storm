// A script to test features, may not always work as expected

import "click_storm_api.d.rhai";
import "ScreenSize"


let t = new_engine();

// Click at with the specified mouse button
//let mb = MouseButton::Left;
//t.click_at(800, 600, mb);

// Add the coordinates to the current mouse position
t.add_position(100, 100);

// Print the screen size
let screen_size = t.get_screen_size();
print(screen_size.to_string());
print(screen_size.center().to_string());

// Enter ctrl + a
//t.set_key(AppKeycode::Control, ButtonDirection::Press);
//t.set_key(AppKeycode::A, ButtonDirection::Press);

//t.set_key(AppKeycode::Control, ButtonDirection::Release);
//t.set_key(AppKeycode::A, ButtonDirection::Release);