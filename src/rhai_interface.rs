#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use device_query::DeviceState;
use enigo::{Button, Enigo, Mouse, Settings};
use rhai::Engine;

// Implementation notes:
//  - The click loop will be re-implemented in Rhai script
//  - The UI will behave the same except push the settings to the Rhai script instead of a custom thread
//    - The internal scripts will be included as bytes to ensure they are not tampered with
//    - See Scope and push_constant
//  - Users can write their own scripts to control input
//  - There will need to be more type conversions than initially expected.

#[derive(Debug, Clone)]
pub struct RhaiInterface {
    engine: Arc<Mutex<Engine>>,
    enigo: Arc<Mutex<Enigo>>,
    device: DeviceState,
}

impl RhaiInterface {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
            panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
        });

        Self {
            engine: Arc::new(Mutex::new(Engine::new())),
            enigo: Arc::new(Mutex::new(enigo)),
            device: DeviceState::new(),
        }
    }

    #[cfg(debug_assertions)]
    pub fn test_hello(&mut self) {
        let engine = self.engine.lock().unwrap();

        engine.run(r#"print("hello, world!")"#).unwrap();

        let result = engine.eval::<i64>("40 + 2").unwrap();
        println!("Result: {}", result);
    }

    #[cfg(debug_assertions)]
    pub fn test_click_at(&mut self) {
        let engine = self.engine.lock().unwrap();

        let script = r#"
            //click_at(100, 100, Button::Left);
            let t = new_rh();
            t.click_at(800, 600);
        "#;
        engine.run(script).unwrap();
    }

    /// Initialize the Rhai engine with the necessary functions and types.
    pub fn initialize(&mut self) {
        let mut engine = self.engine.lock().unwrap();
        // Register the mouse button enum
        engine.register_type::<Button>();
        engine.register_type::<RhaiInterface>();

        engine.register_fn("new_rh", RhaiInterface::new);
        engine.register_fn("click_at", RhaiInterface::click_at);
    }

    fn click_at(&mut self, x: i32, y: i32) {
        //, mouse_button: Button) {
        let mut enigo = self.enigo.lock().unwrap();
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
        //let _ = self.enigo.button(mouse_button, enigo::Direction::Click);
        //println!("Clicking at ({}, {}) with button {:?}", x, y, mouse_button);
        println!("Clicking at ({}, {}) ", x, y);
    }

    // TODO
    //pub fn execute_click_storm(&mut self, app_settings: AppSettings) {}
}
