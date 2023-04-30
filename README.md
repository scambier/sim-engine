## Run the game

```sh
# Start the engine
$ cd ..
$ RUST_LOG=sim_engine cargo run
```

```sh
# Build for the web, on WSL Debian
$ cargo build --target=wasm32-unknown-emscripten --jobs 3
```