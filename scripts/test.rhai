// A script to test features, may not always work as expected

// Create a new click storm object
let cs = new_click_storm();

const SLEEP_TIME = 0.25;
const USE_SLEEP = false;

// Click at with the specified mouse button
//let mb = MouseButton::Left;
//cs.click_at(800, 600, mb);

if USE_SLEEP {
    sleep(SLEEP_TIME);
}

// Add the coordinates to the current mouse position
cs.add_position(100, 100);

if USE_SLEEP {
    sleep(SLEEP_TIME);
}

// Print the screen size
let screen_size = cs.get_screen_size();
print(screen_size.to_string());
print(screen_size.center().to_string());

// Enter ctrl + a
//cs.set_key(AppKeycode::Control, ButtonDirection::Press);
//cs.set_key(AppKeycode::A, ButtonDirection::Press);

//cs.set_key(AppKeycode::Control, ButtonDirection::Release);
//cs.set_key(AppKeycode::A, ButtonDirection::Release);