use enigo::{Button, Enigo, Keyboard, Mouse, Settings};
use rand::Rng;
use std::sync::{Arc, Mutex};

use super::screen_size::ScreenSize;
use crate::input::{
    button_direction::ButtonDirection, keycode::AppKeycode, mouse_button::MouseButton,
};

#[derive(Debug, Clone)]
pub struct ClickStormInterface {
    enigo: Arc<Mutex<Enigo>>,
    //device: DeviceState,
    rng: rand::rngs::ThreadRng,
    screen_size: Option<ScreenSize>,
}

impl ClickStormInterface {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
            panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
        });

        Self {
            enigo: Arc::new(Mutex::new(enigo)),
            //device: DeviceState::new(),
            rng: rand::thread_rng(),
            screen_size: None,
        }
    }

    /**************************************************************************
     * Scripting API
     **************************************************************************/
    // Note: I am pretty sure calling an API function from another API function will cause issues

    /// Click at the specified coordinates with the specified mouse button.
    pub(super) fn click_at(&mut self, x: i32, y: i32, mouse_button: MouseButton) {
        let mut enigo = self.enigo.lock().unwrap();
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
        let _ = enigo.button(mouse_button.into(), enigo::Direction::Click);
    }

    // Click within the specified specified area, X/Y starts at the top left corner.
    pub(super) fn click_within(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        mouse_button: MouseButton,
    ) {
        let mut enigo = self.enigo.lock().unwrap();

        let rand_coords = (
            self.rng.gen_range(x..=x + width),
            self.rng.gen_range(y..=y + height),
        );

        let _ = enigo.move_mouse(rand_coords.0, rand_coords.1, enigo::Coordinate::Abs);
        let _ = enigo.button(mouse_button.into(), enigo::Direction::Click);
    }

    /// Move the mouse to the specified coordinates.
    pub(super) fn move_mouse_to(&mut self, x: i32, y: i32) {
        let mut enigo = self.enigo.lock().unwrap();
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
    }

    /// Adds the coordinates to the current mouse position.
    pub(super) fn add_position(&mut self, x: i32, y: i32) {
        let mut enigo = self.enigo.lock().unwrap();
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Rel);
    }

    /// Drags from the current mouse position to the specified coordinates.
    pub(super) fn drag_to(&mut self, x: i32, y: i32, mouse_button: MouseButton) {
        let mut enigo = self.enigo.lock().unwrap();
        let button: Button = mouse_button.clone().into();
        let _ = enigo.button(button, enigo::Direction::Press);
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
        let _ = enigo.button(button, enigo::Direction::Release);
    }

    /// Drags from the specified coordinates to the specified coordinates.
    pub(super) fn drag_from_to(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        mouse_button: MouseButton,
    ) {
        let mut enigo = self.enigo.lock().unwrap();
        let button: Button = mouse_button.clone().into();
        let _ = enigo.move_mouse(x1, y1, enigo::Coordinate::Abs);
        let _ = enigo.button(button, enigo::Direction::Press);
        let _ = enigo.move_mouse(x2, y2, enigo::Coordinate::Abs);
        let _ = enigo.button(button, enigo::Direction::Release);
    }

    pub(super) fn set_key(&mut self, key: AppKeycode, direction: ButtonDirection) {
        let mut enigo = self.enigo.lock().unwrap();

        let _ = enigo.key(key.into(), direction.into());
    }

    /// Get the screen size.
    pub(super) fn get_screen_size(&mut self) -> ScreenSize {
        match &self.screen_size {
            Some(size) => size.clone(),
            None => {
                let enigo = self.enigo.lock().unwrap();
                let screen_size = enigo
                    .main_display()
                    .unwrap_or_else(|_| panic!("Failed to get screen size."));

                let size: ScreenSize = screen_size.into();
                self.screen_size = Some(size.clone());

                size.clone()
            }
        }
    }

    /// Get a random number within the specified range (min inclusive, max inclusive).
    pub(super) fn rand_range(&mut self, min: i32, max: i32) -> i32 {
        self.rng.gen_range(min..=max)
    }

    /// Get a random number within the specified range (min inclusive, max exclusive).
    pub(super) fn rand_range_excl(&mut self, min: i32, max: i32) -> i32 {
        self.rng.gen_range(min..max)
    }

    /// Get a random boolean value (50/50).
    pub(super) fn rand_bool(&mut self) -> bool {
        self.rng.gen_bool(0.5)
    }

    /**************************************************************************
     * End of Scripting API
     **************************************************************************/
}
