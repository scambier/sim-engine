## Docs

https://docs.rs/boa_engine/latest/boa_engine/

## Run the game

```sh
# Build the game 
$ cd ./game
$ esbuild main.ts --bundle --platform=node --outfile=build/game.js --tree-shaking=false --watch

# Start the engine
$ RUST_LOG=sim_engine cargo run
```

## Running on the Web

Build the project and start a local server to host it:

```bash
$ mkdir -p ./target/web/
$ cp ./index.html ./target/web/

# Release
$ cargo build --release --package sim-engine --target wasm32-unknown-unknown --features web
$ wasm-bindgen --target web --no-typescript --out-dir ./target/web/ ./target/wasm32-unknown-unknown/release/sim-engine.wasm
```

## Benchmarks

### Rects to reach 50fps

- 1200 rects in release mode (TIC-80: 8200)
- 750 rects in dev mode

### Benchmark code

```ts
let framerate = 0
let frames = 0

function init() {
  trace(`Init - ${new Date()}`)
}

class Rect {
  dx: number
  dy: number
  color: number

  constructor(public x: number, public y: number, public width: number, public height: number) {
    this.dx = Math.random() * 2 - 1
    this.dy = Math.random() * 2 - 1
    // Random int between 0 and 15
    this.color = Math.floor(Math.random() * 16)
  }

  public update() {
    this.x += this.dx
    this.y += this.dy
    // Bounce off walls
    if (this.x < 0) { this.dx = Math.abs(this.dx) }
    if (this.x + this.width > WIDTH) { this.dx = -Math.abs(this.dx) }
    if (this.y < 0) { this.dy = Math.abs(this.dy) }
    if (this.y + this.height > HEIGHT) { this.dy = -Math.abs(this.dy) }
  }
}

const rects: Rect[] = []

function update() {
  ++frames

  if (frames % 30 === 0) {
    framerate = Math.round(1 / getDelta())
  }

  if (framerate > 50) {
    rects.push(new Rect(
      Math.random() * WIDTH,
      Math.random() * HEIGHT,
      16,
      16))
  }


  clearScreen()

  for (const rect of rects) {
    rect.update()
    drawRectFill(rect.x, rect.y, rect.width, rect.height, rect.color)
  }

  print(`${rects.length}`, 1, 1, 0, 15)
  print(framerate, WIDTH - 20, 1, 0, 15)
}
```

## Credits

Some parts of this code are taken from Bevy