use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use cs_hal::input::mouse_click::MouseClickType;
use device_query::{DeviceQuery, DeviceState};
use enigo::{Button, Enigo, Mouse, Settings};
use rand::Rng;

use crate::settings::{
    app_settings::AppSettings, cursor_position::CursorPosition, repeat_type::RepeatType,
};

pub fn worker_thread(settings: AppSettings, is_running: Arc<AtomicBool>) {
    // TODO: This is a total mess, clean it up

    // Start the click storm
    println!("Starting click storm");

    // Create instances needed for hardware input
    let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
           panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
       });
    let device = DeviceState::new();

    // Get the time interval to sleep between clicks
    let sleep_duration = settings.click_interval();

    // Random number generator
    let mut rand = rand::thread_rng();

    // Get the mouse button to click with
    let mouse_button = settings.mouse_button().into();

    // Used to keep track of the number of clicks (if repeat type is count)
    let mut current_count = 0;

    let move_mouse = *settings.cursor_position_type() != CursorPosition::CurrentLocation;
    let single_click = *settings.click_type() == MouseClickType::Single;

    let turbo_mode = *settings.repeat_type() == RepeatType::Turbo;

    // Function to click the mouse
    let click_mouse = |enigo: &mut Enigo,
                       mouse_button: Button,
                       location: (i32, i32),
                       move_mouse: bool,
                       single_click: bool| {
        if move_mouse {
            let _ = enigo.move_mouse(location.0, location.1, enigo::Coordinate::Abs);
        }

        // TODO: Handle error
        if single_click {
            let _ = enigo.button(mouse_button, enigo::Direction::Click);
        } else {
            let _ = enigo.button(mouse_button, enigo::Direction::Click);
            let _ = enigo.button(mouse_button, enigo::Direction::Click);
        }
    };

    while is_running.load(Ordering::SeqCst) {
        //println!("Working");

        // Coordinates are in absolute screen coordinates
        let mouse_position = match settings.cursor_position_type() {
            CursorPosition::CurrentLocation => {
                // TODO: Error handling
                enigo
                    .location()
                    .unwrap_or_else(|_| panic!("Failed to get mouse location."))
            }
            CursorPosition::FixedLocation(x, y) => (*x, *y),
        };

        match settings.repeat_type() {
            RepeatType::Repeat(count) => {
                //println!("Count click");
                if current_count >= *count {
                    is_running.store(false, Ordering::SeqCst);
                } else {
                    current_count += 1;

                    click_mouse(
                        &mut enigo,
                        mouse_button,
                        mouse_position,
                        move_mouse,
                        single_click,
                    );
                }
            }
            RepeatType::RepeatUntilStopped => {
                //println!("Repeat click");
                click_mouse(
                    &mut enigo,
                    mouse_button,
                    mouse_position,
                    move_mouse,
                    single_click,
                );
            }
            RepeatType::Turbo => {
                // TODO: Check if this works with left handed mice
                if device.get_mouse().button_pressed[1] {
                    //println!("Turbo click");
                    let _ = enigo.button(mouse_button, enigo::Direction::Release);
                    let _ = enigo.button(mouse_button, enigo::Direction::Press);
                }
            }
        }

        // Sleep for the specified interval
        if *settings.repeat_variation() > 0 {
            let variation = rand.gen_range(0..*settings.repeat_variation() as u64);
            let sleep_duration = sleep_duration + std::time::Duration::from_millis(variation);

            thread::sleep(sleep_duration);
        } else if turbo_mode {
            let sleep_duration =
                std::time::Duration::from_millis(settings.click_interval_milliseconds());
            thread::sleep(sleep_duration);
        } else {
            thread::sleep(sleep_duration);
        }
    }
}
