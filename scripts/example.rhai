// Create a new Click Storm object
let cs = new_click_storm();

// Get the screen center
let screen_center = cs.get_screen_size().center();

print("Screen size: " + screen_center.to_string());

// Move the mouse to the center of the screen
cs.move_mouse_to(screen_center);

sleep(0.1);

// Move right/down 100 pixels
cs.add_position(100, 100);

sleep(0.1);

// Move left/up 200 pixels
cs.add_position(-200, -200);

sleep(0.1);

// If the random boolean is true
if cs.rand_bool() == true {
	// Generate a random number
	let rand = cs.rand_range(-100, 100);

	// Add the random number to the mouse coordinates
	cs.add_position(rand, rand);
}

sleep(0.5);

// Print the mouse position
print(cs.get_mouse_position());

// Move the mouse cursor to the center again
cs.move_mouse_to(screen_center.width, screen_center.height);