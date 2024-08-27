use rhai::Engine;

#[derive(Debug)]
pub struct RhaiInterface {
    engine: Engine,
}

impl RhaiInterface {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    #[cfg(debug_assertions)]
    pub fn test_hello(&mut self) {
        self.engine.run(r#"print("hello, world!")"#).unwrap();
        let result = self.engine.eval::<i64>("40 + 2").unwrap();

        println!("The Answer: {result}"); // prints 42
    }
}
