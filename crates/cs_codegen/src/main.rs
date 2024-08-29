#[cfg(not(feature = "scripting"))]
fn main() {
    println!("Scripting disabled, nothing to do.");
}

// TODO: Add command line features
#[cfg(feature = "scripting")]
fn main() {
    use cs_scripting::rhai_interface::RhaiInterface;

    let interface = RhaiInterface::new();
    let engine = interface.get_engine();
    let engine = engine.lock().unwrap();

    engine
        .definitions()
        .with_headers(true)
        .include_standard_packages(false)
        .write_to_file("click_storm_api.d.rhai")
        .unwrap();
}
