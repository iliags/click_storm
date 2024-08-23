# Click Storm

Click Storm is a safe, open-source, auto-clicker.

## Notes

### Dependencies

Linux users may need to install `libxdo-dev` to run the application.

### Permission

Some devices may prevent hardware access due to permissions, see this [permissions](https://github.com/enigo-rs/enigo/blob/main/Permissions.md) page if you have issues.

## Virus Total

TODO

## Building

 Install [rust](https://www.rust-lang.org/tools/install)

- Open a terminal in the repository location
  - Most operating systems have an option to open a terminal in the current folder when right clicking in the file browser.
- Enter ```cargo build --release``` to build but not execute, ```cargo run --release``` to build and execute
  - The ```--release``` flag can be removed to build the debug version

### Linux

In some cases, linux may also require the following dependencies:

- egui: `sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`
- enigo: `sudo apt install -y libxdo-dev`
- device_query: `sudo apt-get install -y libx11-dev`

### Documentation

```cargo doc --workspace --no-deps```

## Testing

### Unit/Integration Tests

```cargo test --workspace```

### MIRI

See [miri repository](https://github.com/rust-lang/miri).

- Install the nightly toolchain with ```rustup toolchain install nightly```
- Install miri component on rust nightly: ```rustup +nightly component add miri```
- Run tests with miri: ```cargo +nightly miri test --workspace```
- Run application with miri: ```cargo +nightly miri run```
  - This takes a long time to run and currently fails due to an unsupported operation in `winit`.
