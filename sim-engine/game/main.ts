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