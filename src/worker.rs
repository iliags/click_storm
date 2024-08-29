use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
        Arc,
    },
    thread::{self, JoinHandle},
};

use cs_hal::input::mouse_click::MouseClickType;
use device_query::{DeviceQuery, DeviceState};
use enigo::{Button, Enigo, Mouse, Settings};
use rand::Rng;

use crate::settings::{
    app_settings::AppSettings, cursor_position::CursorPosition, repeat_type::RepeatType,
};

#[derive(Debug, Clone)]
pub enum ClickStormMessage {
    Start(AppSettings, Arc<AtomicBool>),
    Stop,
    Shutdown,
}

pub fn worker_thread(receiver: Receiver<ClickStormMessage>) {
    // TODO: This is a total mess, clean it up
    let mut thread: Option<JoinHandle<()>> = None;
    let mut is_working = Arc::new(AtomicBool::new(false));

    loop {
        match receiver.recv() {
            Ok(message) => {
                match message {
                    ClickStormMessage::Start(settings, is_running) => {
                        // Start the click storm
                        println!("Starting click storm");

                        if let Some(thread) = thread.take() {
                            is_running.store(true, Ordering::SeqCst);
                            let _ = thread.join();
                        }

                        // Inner thread atomic
                        let doing_work = Arc::clone(&is_running);

                        // Worker thread atomic
                        is_working = Arc::clone(&is_running);
                        is_running.store(true, Ordering::SeqCst);

                        let settings_clone = Arc::clone(&Arc::new(settings));

                        // Worker thread
                        thread = Some(thread::spawn(move || {
                            let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
                            panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
                        });

                            // Get the time interval to sleep between clicks
                            let sleep_duration = settings_clone.click_interval();

                            // Random number generator
                            let mut rand = rand::thread_rng();

                            // Get the mouse button to click with
                            let mouse_button = settings_clone.mouse_button().into();

                            let mut current_count = 0;

                            let move_mouse = *settings_clone.cursor_position_type()
                                != CursorPosition::CurrentLocation;
                            let single_click =
                                *settings_clone.click_type() == MouseClickType::Single;

                            let turbo_mode = *settings_clone.repeat_type() == RepeatType::Turbo;
                            let device = DeviceState::new();

                            // Function to click the mouse
                            let click_mouse =
                                |enigo: &mut Enigo,
                                 mouse_button: Button,
                                 location: (i32, i32),
                                 move_mouse: bool,
                                 single_click: bool| {
                                    if move_mouse {
                                        let _ = enigo.move_mouse(
                                            location.0,
                                            location.1,
                                            enigo::Coordinate::Abs,
                                        );
                                    }

                                    // TODO: Handle error
                                    if single_click {
                                        let _ = enigo.button(mouse_button, enigo::Direction::Click);
                                    } else {
                                        let _ = enigo.button(mouse_button, enigo::Direction::Click);
                                        let _ = enigo.button(mouse_button, enigo::Direction::Click);
                                    }
                                };

                            while doing_work.load(Ordering::SeqCst) {
                                //println!("Working");

                                // Coordinates are in absolute screen coordinates
                                let mouse_position = match settings_clone.cursor_position_type() {
                                    CursorPosition::CurrentLocation => {
                                        // TODO: Error handling
                                        enigo.location().unwrap_or_else(|_| {
                                            panic!("Failed to get mouse location.")
                                        })
                                    }
                                    CursorPosition::FixedLocation(x, y) => (*x, *y),
                                };

                                match settings_clone.repeat_type() {
                                    RepeatType::Repeat(count) => {
                                        //println!("Count click");
                                        if current_count >= *count {
                                            doing_work.store(false, Ordering::SeqCst);
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
                                            let _ = enigo
                                                .button(mouse_button, enigo::Direction::Release);
                                            let _ =
                                                enigo.button(mouse_button, enigo::Direction::Press);
                                        }
                                    }
                                }

                                if *settings_clone.repeat_variation() > 0 {
                                    let variation = rand
                                        .gen_range(0..*settings_clone.repeat_variation() as u64);
                                    let sleep_duration = sleep_duration
                                        + std::time::Duration::from_millis(variation);

                                    thread::sleep(sleep_duration);
                                } else if turbo_mode {
                                    let sleep_duration = std::time::Duration::from_millis(
                                        settings_clone.click_interval_milliseconds(),
                                    );
                                    thread::sleep(sleep_duration);
                                } else {
                                    thread::sleep(sleep_duration);
                                }
                            }
                        }));
                    }
                    ClickStormMessage::Stop => {
                        // Stop the click storm
                        if let Some(thread) = thread.take() {
                            println!("Stopping click storm");
                            is_working.store(false, Ordering::SeqCst);
                            let _ = thread.join();
                        }
                    }
                    ClickStormMessage::Shutdown => {
                        // Shutdown the thread
                        if let Some(thread) = thread.take() {
                            println!("Shutting down click storm thread");
                            is_working.store(false, Ordering::SeqCst);
                            let _ = thread.join();
                        }
                        break;
                    }
                }
            }

            Err(e) => {
                println!("Error receiving message: {:?}", e);
                break;
            }
        }
    }
}
