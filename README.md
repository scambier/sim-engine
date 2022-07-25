## Run the game

```sh
# Build the game 
$ cd ./sim-engine/game
$ esbuild main.ts --bundle --platform=node --outfile=build/game.js --tree-shaking=false 

# Start the engine
$ cd ..
$ RUST_LOG=sim_engine cargo run
```
