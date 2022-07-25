## Run the game

```sh
# Build the game 
$ cd ./sim-engine/game
$ esbuild main.ts --bundle --platform=node --outfile=build/game.js --tree-shaking=false --watch

# Start the engine
$ cd ./sim-engine
$ RUST_LOG=sim-engine cargo run
```
